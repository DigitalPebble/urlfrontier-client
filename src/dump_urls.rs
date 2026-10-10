//! dump-urls: exports the URLs as JSON, one per line, in the format taken by put-urls

use std::collections::VecDeque;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufWriter, Stdout, Write};
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Instant;

use flate2::write::GzEncoder;
use flate2::Compression;
use tonic::Request;

use crate::frontier::{ListUrlParams, Local, Pagination, UrlItem};
use crate::Client;

/// how many URLs between progress messages
const REPORT_EVERY: u64 = 100_000;

/// how many queue names are retrieved per ListQueues call
const QUEUE_PAGE: u32 = 10_000;

/// asks for all the URLs; the service is in Java and reads the size as a signed int
const ALL_URLS: u32 = i32::MAX as u32;

type DumpResult<T> = Result<T, Box<dyn Error + Send + Sync>>;

/// Dumps the URLs of every crawl, or of `crawl_id` only, into `output` or the standard
/// output; returns false if the dump is incomplete
pub async fn run(
    mut client: Client,
    address: String,
    output: Option<PathBuf>,
    crawl_id: Option<String>,
    threads: usize,
) -> bool {
    // the URLs come from the node we are connected to, so take the crawl IDs from
    // that same node
    let crawl_ids = match crawl_id {
        Some(crawl_id) => vec![crawl_id],
        None => match client
            .list_crawls(Request::new(Local { local: true }))
            .await
        {
            Ok(crawls) => crawls.into_inner().values,
            Err(status) => {
                eprintln!("Error while dumping the URLs: {}", status.message());
                return false;
            }
        },
    };

    let destination = match &output {
        Some(path) => path.display().to_string(),
        None => "the standard output".to_string(),
    };
    let out = match Output::open(output) {
        Ok(out) => out,
        Err(e) => {
            eprintln!("Error while writing {}: {}", destination, e);
            return false;
        }
    };
    let dump = Arc::new(Dump {
        out: Mutex::new(Some(out)),
        total: AtomicU64::new(0),
    });

    let start = Instant::now();
    let mut success = if threads > 1 {
        dump_parallel(client, address, crawl_ids, threads, dump.clone()).await
    } else {
        dump_sequential(client, crawl_ids, &dump).await
    };

    // whatever happened, finish the output so that what was dumped so far stays usable:
    // a gzip stream without its trailer would be invalid
    let out = dump.out.lock().unwrap().take();
    if let Some(Err(e)) = out.map(Output::finish) {
        success = false;
        eprintln!("Error while writing {}: {}", destination, e);
    }

    if !success {
        eprintln!("The dump is incomplete");
        return false;
    }
    eprintln!(
        "Total: {} URLs dumped in {} msec",
        dump.total.load(Ordering::Relaxed),
        start.elapsed().as_millis()
    );
    true
}

/// Streams each crawl whole, in a single call
async fn dump_sequential(mut client: Client, crawl_ids: Vec<String>, dump: &Dump) -> bool {
    for crawl_id in crawl_ids {
        let params = ListUrlParams {
            crawl_id: crawl_id.clone(),
            size: ALL_URLS,
            ..Default::default()
        };
        match dump.urls(&mut client, params).await {
            Ok(count) => eprintln!("{} URLs dumped for crawl {}", count, crawl_id),
            Err(e) => {
                eprintln!("Error while dumping the URLs: {}", e);
                return false;
            }
        }
    }
    true
}

/// Lists the queues of every crawl, then dumps them queue by queue with the connections
/// sharing the list; returns false if any of them failed
async fn dump_parallel(
    mut client: Client,
    address: String,
    crawl_ids: Vec<String>,
    threads: usize,
    dump: Arc<Dump>,
) -> bool {
    // (crawlID, queue) pairs, taken by whichever connection is free
    let mut work = VecDeque::new();
    for crawl_id in crawl_ids {
        match list_queues(&mut client, &crawl_id).await {
            Ok(queues) => {
                eprintln!("{} queues to dump for crawl {}", queues.len(), crawl_id);
                work.extend(queues.into_iter().map(|queue| (crawl_id.clone(), queue)));
            }
            Err(e) => {
                eprintln!("Error while dumping the URLs: {}", e);
                return false;
            }
        }
    }
    let work = Arc::new(Mutex::new(work));
    let failed = Arc::new(AtomicBool::new(false));

    let workers: Vec<_> = (0..threads)
        .map(|_| {
            let address = address.clone();
            let work = work.clone();
            let failed = failed.clone();
            let dump = dump.clone();
            tokio::spawn(async move {
                let result = async {
                    let mut client = Client::connect(address).await?;
                    while !failed.load(Ordering::Relaxed) {
                        let Some((crawl_id, key)) = work.lock().unwrap().pop_front() else {
                            break;
                        };
                        let params = ListUrlParams {
                            crawl_id,
                            key,
                            size: ALL_URLS,
                            ..Default::default()
                        };
                        dump.urls(&mut client, params).await?;
                    }
                    DumpResult::Ok(())
                }
                .await;
                if let Err(e) = result {
                    failed.store(true, Ordering::Relaxed);
                    eprintln!("Error while dumping the URLs: {}", e);
                }
            })
        })
        .collect();

    for worker in workers {
        if worker.await.is_err() {
            failed.store(true, Ordering::Relaxed);
        }
    }
    !failed.load(Ordering::Relaxed)
}

/// Returns all the queues of a crawl on the node we are connected to, including the inactive ones
async fn list_queues(client: &mut Client, crawl_id: &str) -> DumpResult<Vec<String>> {
    let mut queues = Vec::new();
    loop {
        let page = client
            .list_queues(Request::new(Pagination {
                start: queues.len() as u32,
                size: QUEUE_PAGE,
                include_inactive: true,
                crawl_id: crawl_id.to_string(),
                local: true,
            }))
            .await?
            .into_inner();
        let size = page.values.len();
        queues.extend(page.values);
        if size < QUEUE_PAGE as usize {
            return Ok(queues);
        }
    }
}

/// The output of the dump, shared by the connections
struct Dump {
    /// taken out when the dump is finished
    out: Mutex<Option<Output>>,
    total: AtomicU64,
}

impl Dump {
    /// Writes the URLs returned by ListURLs; returns how many there were
    async fn urls(&self, client: &mut Client, params: ListUrlParams) -> DumpResult<u64> {
        let mut stream = client.list_ur_ls(Request::new(params)).await?.into_inner();
        let mut count = 0;
        while let Some(item) = stream.message().await? {
            self.write(&item)?;
            count += 1;
        }
        Ok(count)
    }

    fn write(&self, item: &UrlItem) -> DumpResult<()> {
        let line = serde_json::to_string(item)?;
        {
            let mut out = self.out.lock().unwrap();
            let out = out.as_mut().ok_or("the dump has been finished")?;
            out.write_all(line.as_bytes())?;
            out.write_all(b"\n")?;
        }
        let so_far = self.total.fetch_add(1, Ordering::Relaxed) + 1;
        if so_far % REPORT_EVERY == 0 {
            eprintln!("{} URLs dumped so far...", so_far);
        }
        Ok(())
    }
}

enum Output {
    Stdout(BufWriter<Stdout>),
    File(BufWriter<File>),
    Gzip(GzEncoder<BufWriter<File>>),
}

impl Output {
    /// Opens the file, compressed with gzip if its name ends in .gz, or the standard output
    fn open(path: Option<PathBuf>) -> io::Result<Output> {
        let Some(path) = path else {
            return Ok(Output::Stdout(BufWriter::new(io::stdout())));
        };
        let file = BufWriter::new(File::create(&path)?);
        if path.extension().is_some_and(|ext| ext == "gz") {
            Ok(Output::Gzip(GzEncoder::new(file, Compression::default())))
        } else {
            Ok(Output::File(file))
        }
    }

    fn finish(self) -> io::Result<()> {
        match self {
            Output::Stdout(mut out) => out.flush(),
            Output::File(mut out) => out.flush(),
            Output::Gzip(out) => out.finish()?.flush(),
        }
    }
}

impl Write for Output {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match self {
            Output::Stdout(out) => out.write(buf),
            Output::File(out) => out.write(buf),
            Output::Gzip(out) => out.write(buf),
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        match self {
            Output::Stdout(out) => out.flush(),
            Output::File(out) => out.flush(),
            Output::Gzip(out) => out.flush(),
        }
    }
}
