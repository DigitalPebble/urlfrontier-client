//! put-urls: sends the URLs of a file to the Frontier, on one or more streams

use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

use flate2::read::MultiGzDecoder;
use tokio::sync::{mpsc, Mutex, Semaphore};
use tokio_stream::wrappers::ReceiverStream;

use crate::frontier::ack_message::Status;
use crate::frontier::{url_item, DiscoveredBatch, DiscoveredUrlItem, UrlInfo, UrlItem};
use crate::Client;

/// how often the intermediate throughput is displayed
const REPORT_INTERVAL: Duration = Duration::from_secs(30);

/// how many lines a stream takes from the input at once, so that they are not all
/// queueing on the reader for every single line
const CHUNK_SIZE: usize = 512;

/// messages waiting to be written on a stream; the actual flow control is done by
/// the window of URLs not acked yet
const STREAM_BUFFER: usize = 64;

/// the outcome of the URLs sent, across all the streams
#[derive(Default)]
struct Counters {
    sent: AtomicU64,
    acked: AtomicU64,
    ok: AtomicU64,
    skipped: AtomicU64,
    failed: AtomicU64,
    /// set when any of the streams is terminated by an error
    stream_error: AtomicBool,
    read_error: AtomicBool,
}

impl Counters {
    fn record(&self, status: i32) {
        self.acked.fetch_add(1, Ordering::Relaxed);
        let counter = match Status::from_i32(status) {
            Some(Status::Ok) => &self.ok,
            Some(Status::Skipped) => &self.skipped,
            Some(Status::Fail) => &self.failed,
            None => return,
        };
        counter.fetch_add(1, Ordering::Relaxed);
    }
}

/// consecutive lines of the input
struct Chunk {
    first_line_number: usize,
    lines: Vec<String>,
}

/// Sends the URLs of `file` to the Frontier on `threads` streams; returns false if the input
/// could not be read or a stream failed
pub async fn run(
    address: String,
    file: PathBuf,
    crawl_id: String,
    threads: usize,
    in_flight: usize,
    batch: usize,
) -> bool {
    let streams = threads.max(1);
    // how far a stream is allowed to run ahead of the Frontier, counted on its own so
    // that a slow stream does not hold the others back
    let window = in_flight.max(1);

    let mut batch = batch;
    if batch > 0 && !supports_put_discovered(&address).await {
        eprintln!("Server does not support PutDiscovered - sending the URLs individually");
        batch = 0;
    }

    let reader = match open_input(&file) {
        Ok(reader) => reader,
        Err(e) => {
            eprintln!("Error while reading {}: {}", file.display(), e);
            return false;
        }
    };

    let counters = Arc::new(Counters::default());
    let start = Instant::now();
    let reporter = tokio::spawn(report(counters.clone()));

    // the file is read once and the lines handed out in chunks, so that adding streams
    // multiplies the parsing and sending but not the reading
    let (chunk_tx, chunk_rx) = mpsc::channel(streams * 2);
    let reader_counters = counters.clone();
    let reader_task = tokio::task::spawn_blocking(move || {
        read_chunks(reader, chunk_tx, &reader_counters, &file);
    });

    let chunks = Arc::new(Mutex::new(chunk_rx));
    let senders: Vec<_> = (0..streams)
        .map(|_| {
            tokio::spawn(send_from(
                address.clone(),
                chunks.clone(),
                crawl_id.clone(),
                window,
                batch,
                counters.clone(),
            ))
        })
        .collect();
    // the reader stops once the senders are gone and nothing takes the chunks anymore
    drop(chunks);
    for sender in senders {
        if sender.await.is_err() {
            counters.stream_error.store(true, Ordering::Relaxed);
        }
    }
    let _ = reader_task.await;
    reporter.abort();

    let elapsed = start.elapsed();
    let sent = counters.sent.load(Ordering::Relaxed);
    let acked = counters.acked.load(Ordering::Relaxed);
    println!("Sent: {}", sent);
    println!("Acked: {}", acked);
    println!("OK: {}", counters.ok.load(Ordering::Relaxed));
    println!("Skipped: {}", counters.skipped.load(Ordering::Relaxed));
    println!("Failed: {}", counters.failed.load(Ordering::Relaxed));
    println!("Total time: {} msec", elapsed.as_millis());
    println!(
        "Average OPS: {:.2}",
        acked as f64 / elapsed.as_secs_f64().max(0.001)
    );

    let stream_error = counters.stream_error.load(Ordering::Relaxed);
    if stream_error {
        eprintln!(
            "The connection to the Frontier failed, {} URLs out of {} sent were not confirmed",
            sent - acked,
            sent
        );
    }
    !stream_error && !counters.read_error.load(Ordering::Relaxed)
}

fn open_input(path: &Path) -> std::io::Result<Box<dyn BufRead + Send>> {
    let file = File::open(path)?;
    let input: Box<dyn Read + Send> = if path.extension().is_some_and(|ext| ext == "gz") {
        Box::new(MultiGzDecoder::new(file))
    } else {
        Box::new(file)
    };
    Ok(Box::new(BufReader::with_capacity(1 << 16, input)))
}

/// Reads the input and hands it out in chunks until it is exhausted or nobody takes the chunks
fn read_chunks(
    reader: Box<dyn BufRead + Send>,
    chunks: mpsc::Sender<Chunk>,
    counters: &Counters,
    file: &Path,
) {
    let mut lines = reader.lines();
    let mut line_number = 1;
    let mut exhausted = false;
    while !exhausted {
        let mut chunk = Vec::with_capacity(CHUNK_SIZE);
        while chunk.len() < CHUNK_SIZE {
            match lines.next() {
                Some(Ok(line)) => chunk.push(line),
                Some(Err(e)) => {
                    counters.read_error.store(true, Ordering::Relaxed);
                    eprintln!("Error while reading {}: {}", file.display(), e);
                    exhausted = true;
                    break;
                }
                None => {
                    exhausted = true;
                    break;
                }
            }
        }
        if chunk.is_empty() {
            break;
        }
        let size = chunk.len();
        let chunk = Chunk {
            first_line_number: line_number,
            lines: chunk,
        };
        if chunks.blocking_send(chunk).is_err() {
            // all the senders are gone
            break;
        }
        line_number += size;
    }
}

/// Displays the throughput at regular intervals so that any degradation over time is visible
async fn report(counters: Arc<Counters>) {
    let mut interval = tokio::time::interval_at(
        tokio::time::Instant::now() + REPORT_INTERVAL,
        REPORT_INTERVAL,
    );
    let mut last_report = Instant::now();
    let mut last_acked = 0;
    loop {
        interval.tick().await;
        let elapsed = last_report.elapsed();
        last_report = Instant::now();
        let acked = counters.acked.load(Ordering::Relaxed);
        let delta = acked - last_acked;
        last_acked = acked;
        println!(
            "Acked: {} - OPS over the last {} sec: {:.2}",
            acked,
            elapsed.as_secs_f64().round(),
            delta as f64 / elapsed.as_secs_f64()
        );
    }
}

/// Probes the server with an empty batch: an old server answers the call itself with
/// UNIMPLEMENTED, a current one acks it
async fn supports_put_discovered(address: &str) -> bool {
    let probe = async {
        let mut client = Client::connect(address.to_string()).await.ok()?;
        let batch = DiscoveredBatch {
            id: "probe".to_string(),
            items: Vec::new(),
        };
        let mut acks = client
            .put_discovered(tokio_stream::iter(vec![batch]))
            .await
            .ok()?
            .into_inner();
        acks.message().await.ok()?
    };
    matches!(
        tokio::time::timeout(Duration::from_secs(10), probe).await,
        Ok(Some(_))
    )
}

/// Sends URLs taken from the shared chunks on a connection of its own until they are
/// exhausted, then waits for the Frontier to confirm every one of them
async fn send_from(
    address: String,
    chunks: Arc<Mutex<mpsc::Receiver<Chunk>>>,
    crawl_id: String,
    window: usize,
    batch: usize,
    counters: Arc<Counters>,
) {
    let client = match Client::connect(address).await {
        Ok(client) => client,
        Err(e) => {
            counters.stream_error.store(true, Ordering::Relaxed);
            eprintln!("Error while connecting to the Frontier: {}", e);
            return;
        }
    };

    // a permit per URL which can be sent without being confirmed yet; an ack gives it
    // back. Closed when a stream fails, which stops the sending
    let permits = Arc::new(Semaphore::new(window));

    let (item_tx, item_rx) = mpsc::channel(STREAM_BUFFER);
    let item_acks = {
        let mut client = client.clone();
        let permits = permits.clone();
        let counters = counters.clone();
        tokio::spawn(async move {
            let result = async {
                let mut acks = client
                    .put_ur_ls(ReceiverStream::new(item_rx))
                    .await?
                    .into_inner();
                while let Some(ack) = acks.message().await? {
                    counters.record(ack.status);
                    permits.add_permits(1);
                }
                Ok::<_, tonic::Status>(())
            }
            .await;
            if let Err(status) = result {
                stream_failed(&counters, &permits, status);
            }
        })
    };

    // the discovered URLs travel in batches on a PutDiscovered stream of their own when
    // the option is on; known URLs keep using PutURLs
    let mut batches = None;
    if batch > 0 {
        let (batch_tx, batch_rx) = mpsc::channel(STREAM_BUFFER);
        let mut client = client.clone();
        let permits = permits.clone();
        let counters = counters.clone();
        let batch_acks = tokio::spawn(async move {
            let result = async {
                let mut acks = client
                    .put_discovered(ReceiverStream::new(batch_rx))
                    .await?
                    .into_inner();
                while let Some(ack) = acks.message().await? {
                    for status in &ack.statuses {
                        counters.record(*status);
                    }
                    permits.add_permits(ack.statuses.len().min(window));
                }
                Ok::<_, tonic::Status>(())
            }
            .await;
            if let Err(status) = result {
                stream_failed(&counters, &permits, status);
            }
        });
        batches = Some((batch_tx, batch_acks));
    }

    let mut buffer = Vec::with_capacity(batch);
    let mut batch_sequence = 0;
    let mut stopped = false;

    'outer: while !stopped {
        let chunk = chunks.lock().await.recv().await;
        let Some(chunk) = chunk else { break };
        for (i, line) in chunk.lines.iter().enumerate() {
            if line.trim().is_empty() {
                continue;
            }
            let Some(item) = parse(line, &crawl_id) else {
                eprintln!("Invalid input line {}", chunk.first_line_number + i);
                continue;
            };
            if let Some((batch_tx, _)) = &batches {
                if let Some(url_item::Item::Discovered(discovered)) = item.item {
                    buffer.push(discovered.info.unwrap_or_default());
                    if buffer.len() >= batch
                        && !send_batch(
                            &mut buffer,
                            batch_tx,
                            &permits,
                            window,
                            &mut batch_sequence,
                            &counters,
                        )
                        .await
                    {
                        stopped = true;
                        break 'outer;
                    }
                    continue;
                }
            }
            // waits for room in the window, woken by the acks
            match permits.acquire().await {
                Ok(permit) => permit.forget(),
                // the stream is dead, no point in sending anything else
                Err(_) => break 'outer,
            }
            if item_tx.send(item).await.is_err() {
                break 'outer;
            }
            counters.sent.fetch_add(1, Ordering::Relaxed);
        }
    }

    // both streams are closed even if one of them failed: the healthy one only ends once
    // the Frontier has acked everything it received
    drop(item_tx);
    if let Some((batch_tx, batch_acks)) = batches {
        // whatever remains in the buffer goes out before the stream is closed
        if !stopped {
            send_batch(
                &mut buffer,
                &batch_tx,
                &permits,
                window,
                &mut batch_sequence,
                &counters,
            )
            .await;
        }
        drop(batch_tx);
        let _ = batch_acks.await;
    }
    let _ = item_acks.await;
}

/// Waits for room in the window, then sends the buffered URLs as one batch. Returns false when
/// the stream died
async fn send_batch(
    buffer: &mut Vec<UrlInfo>,
    batch_tx: &mpsc::Sender<DiscoveredBatch>,
    permits: &Semaphore,
    window: usize,
    batch_sequence: &mut u64,
    counters: &Counters,
) -> bool {
    if buffer.is_empty() {
        return true;
    }
    // a batch larger than the window takes all of it
    match permits.acquire_many(buffer.len().min(window) as u32).await {
        Ok(permit) => permit.forget(),
        Err(_) => return false,
    }
    *batch_sequence += 1;
    let size = buffer.len();
    let batch = DiscoveredBatch {
        id: batch_sequence.to_string(),
        items: std::mem::replace(buffer, Vec::with_capacity(size)),
    };
    if batch_tx.send(batch).await.is_err() {
        return false;
    }
    counters.sent.fetch_add(size as u64, Ordering::Relaxed);
    true
}

fn stream_failed(counters: &Counters, permits: &Semaphore, status: tonic::Status) {
    counters.stream_error.store(true, Ordering::Relaxed);
    eprintln!("Error while sending the URLs: {}", status.message());
    // wakes up the sender if it is waiting for room in the window
    permits.close();
}

/// Parses a line of input, which is either a URLItem in JSON e.g.
/// `{"discovered":{"info":{"url":"http://test.com","key":"test.com"}}}`
/// or a plain URL, sent as a discovered URL with no metadata, the key left for the server
/// to determine (i.e. hostname) and no explicit refetch date
fn parse(line: &str, crawl_id: &str) -> Option<UrlItem> {
    let line = line.trim();
    if line.starts_with('{') {
        return serde_json::from_str(line).ok();
    }
    let info = UrlInfo {
        url: line.to_string(),
        crawl_id: crawl_id.to_string(),
        ..Default::default()
    };
    Some(UrlItem {
        id: format!("{}_{}", crawl_id, line),
        item: Some(url_item::Item::Discovered(DiscoveredUrlItem {
            info: Some(info),
        })),
        ..Default::default()
    })
}
