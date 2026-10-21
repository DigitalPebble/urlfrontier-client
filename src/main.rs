pub mod frontier {
    tonic::include_proto!("urlfrontier");
    include!(concat!(env!("OUT_DIR"), "/urlfrontier.serde.rs"));
}

mod dump_urls;
mod put_urls;

use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use std::process::ExitCode;
use std::time::Instant;

use chrono::{Local as LocalTime, TimeZone};
use clap::{ArgAction, CommandFactory, Parser, Subcommand};
use clap_complete::{generate, Shell};
use frontier::log_level_params::Level;
use frontier::url_frontier_client::UrlFrontierClient;
use frontier::{
    get_params, url_item, Active, AnyCrawlId, CountUrlParams, CrawlLimitParams, DeleteCrawlMessage,
    Empty, GetParams, ListUrlParams, Local, LogLevelParams, Pagination, PurgeUrlParams,
    QueueWithinCrawlParams, UrlInfo, UrlItem, UrlStatusRequest,
};
use tonic::transport::Channel;
use tonic::{Code, Request};

/// crawl used when none is specified
pub const DEFAULT_CRAWL: &str = "DEFAULT";

pub type Client = UrlFrontierClient<Channel>;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Host of the URLFrontier service to connect to
    #[arg(long, default_value_t = String::from("localhost"))]
    host: String,

    /// Port of the URLFrontier service to connect to
    #[arg(short, long, default_value_t = 7071)]
    port: usize,

    #[command(subcommand)]
    action: Action,
}

#[derive(Subcommand, Debug)]
enum Action {
    /// Prints out the list of nodes forming the cluster
    ListNodes,

    /// Prints out the list of crawls
    ListCrawls {
        /// Restricts the scope to this frontier instance instead of aggregating over the cluster
        #[arg(short, long)]
        local: bool,
    },

    /// Prints out the queues of a crawl
    ListQueues {
        /// Maximum number of queues to return
        #[arg(short, long, default_value_t = 100)]
        number_queues: u32,

        /// Position of the first queue to return
        #[arg(short, long, default_value_t = 0)]
        start: u32,

        /// Output file to dump all the queues into, page after page
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// Include inactive queues in the results
        #[arg(short, long)]
        include_inactive: bool,

        /// Crawl to get the queues for
        #[arg(short, long, default_value = DEFAULT_CRAWL)]
        crawl_id: String,

        /// Restricts the scope to this frontier instance instead of aggregating over the cluster
        #[arg(short, long)]
        local: bool,
    },

    /// Prints out the URLs in the Frontier
    ListUrls {
        /// Maximum number of URLs to return
        #[arg(short, long, default_value_t = 100)]
        number_urls: u32,

        /// Position of the first URL to return
        #[arg(short, long, default_value_t = 0)]
        start: u32,

        /// Key of the queue to list the URLs of
        #[arg(short, long)]
        key: Option<String>,

        /// Output file to dump the URLs into
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// Crawl to list the URLs of
        #[arg(short, long, default_value = DEFAULT_CRAWL)]
        crawl_id: String,

        /// Restricts the scope to this frontier instance instead of aggregating over the cluster
        #[arg(short, long)]
        local: bool,

        /// Outputs the URLs as JSON, one per line
        #[arg(short, long)]
        json: bool,

        /// Prints the refetch date in the local time zone instead of UTC seconds since the epoch;
        /// ignored with --json
        #[arg(short, long)]
        parse_date: bool,

        /// String filter applied to the URLs
        #[arg(short, long)]
        filter: Option<String>,

        /// Ignore case for the filter
        #[arg(short, long)]
        ignore_case: bool,
    },

    /// Prints out stats from the Frontier
    GetStats {
        /// Restricts the stats to a specific queue
        #[arg(short, long)]
        key: Option<String>,

        /// Crawl to get the stats for
        #[arg(short, long, default_value = DEFAULT_CRAWL)]
        crawl_id: String,

        /// Restricts the scope to this frontier instance instead of aggregating over the cluster
        #[arg(short, long)]
        local: bool,
    },

    /// Sends URLs from a file into the Frontier
    #[command(long_about = "Sends URLs from a file into the Frontier.\n\n\
        Each line is either a URL, sent as a discovered URL, or a URLItem in JSON e.g. \
        {\"discovered\":{\"info\":{\"url\":\"http://test.com\",\"key\":\"test.com\"}}}. \
        The two can be mixed in the same file.")]
    PutUrls {
        /// File containing the URLs to send, decompressed with gzip if its name ends in .gz
        #[arg(short, long)]
        file: PathBuf,

        /// Crawl to send the URLs to, for the lines which are plain URLs
        #[arg(short, long, default_value = DEFAULT_CRAWL)]
        crawl_id: String,

        /// Number of streams sending URLs, each on its own connection. The URLs are shared
        /// between them, they are not sent more than once
        #[arg(short, long, default_value_t = 1)]
        threads: usize,

        /// Maximum number of URLs sent on a stream but not confirmed yet
        #[arg(short = 'w', long, default_value_t = 10000)]
        in_flight: usize,

        /// Number of discovered URLs to send per message through PutDiscovered (0 to send
        /// every URL individually through PutURLs). Known URLs always go through PutURLs.
        /// Falls back to sending everything individually if the server has no PutDiscovered
        #[arg(short, long, default_value_t = 0)]
        batch: usize,
    },

    /// Gets URLs from the Frontier and prints them as JSON, one per line
    GetUrls {
        /// Key of the queue to get URLs from
        #[arg(short, long)]
        key: Option<String>,

        /// Delay in seconds before the URLs can be sent again for fetching (0 lets the server
        /// decide)
        #[arg(short, long, default_value_t = 0)]
        delay: u32,

        /// Maximum number of URLs per queue
        #[arg(short, long, default_value_t = 1)]
        max_urls: u32,

        /// Maximum number of queues
        #[arg(short = 'q', long, default_value_t = 1)]
        max_queues: u32,

        /// Crawl to get URLs from; any crawl if not set
        #[arg(short, long)]
        crawl_id: Option<String>,
    },

    /// Pauses or resumes the Frontier
    SetActive {
        /// Whether the Frontier should return URLs or not
        #[arg(default_value_t = true, action = ArgAction::Set)]
        state: bool,

        /// Restricts the scope to this frontier instance instead of the whole cluster
        #[arg(short, long)]
        local: bool,
    },

    /// Checks whether the Frontier has been paused
    GetActive {
        /// Restricts the scope to this frontier instance instead of the whole cluster
        #[arg(short, long)]
        local: bool,
    },

    /// Deletes a queue from the Frontier
    DeleteQueue {
        /// Key of the queue to delete
        #[arg(short, long)]
        key: String,

        /// Crawl the queue belongs to
        #[arg(short, long, default_value = DEFAULT_CRAWL)]
        crawl_id: String,

        /// Restricts the scope to this frontier instance instead of the whole cluster
        #[arg(short, long)]
        local: bool,
    },

    /// Deletes an entire crawl from the Frontier
    DeleteCrawl {
        /// Crawl to delete
        #[arg(short, long)]
        crawl_id: String,

        /// Restricts the scope to this frontier instance instead of the whole cluster
        #[arg(short, long)]
        local: bool,
    },

    /// Changes the log level of a package in the Frontier service
    SetLogLevel {
        /// Package name
        #[arg(short, long)]
        package: String,

        /// Log level
        #[arg(short, long, default_value = "DEBUG",
              value_parser = ["TRACE", "DEBUG", "INFO", "WARN", "ERROR"])]
        level: String,

        /// Restricts the scope to this frontier instance instead of the whole cluster
        #[arg(long)]
        local: bool,
    },

    /// Sets the crawl limit for a queue
    SetCrawlLimit {
        /// Key of the queue to set the limit for
        #[arg(short, long)]
        key: String,

        /// Crawl the queue belongs to
        #[arg(short, long, default_value = DEFAULT_CRAWL)]
        crawl_id: String,

        /// The limit (0 to disable)
        #[arg(short, long, default_value_t = 0)]
        limit: u32,

        /// Restricts the scope to this frontier instance instead of the whole cluster
        #[arg(long)]
        local: bool,
    },

    /// Gets the status of a URL
    GetUrlStatus {
        /// URL to check
        #[arg(short, long)]
        url: String,

        /// Crawl of the URL
        #[arg(short, long, default_value = DEFAULT_CRAWL)]
        crawl_id: String,

        /// Key of the queue of the URL
        #[arg(short, long)]
        key: Option<String>,

        /// Prints the refetch date in the local time zone instead of UTC seconds since the epoch
        #[arg(short, long)]
        parse_date: bool,
    },

    /// Counts the URLs in the Frontier
    CountUrls {
        /// Key of the queue to count the URLs of
        #[arg(short, long)]
        key: Option<String>,

        /// Crawl to count the URLs of
        #[arg(short, long, default_value = DEFAULT_CRAWL)]
        crawl_id: String,

        /// Restricts the scope to this frontier instance instead of aggregating over the cluster
        #[arg(short, long)]
        local: bool,

        /// String filter applied to the URLs
        #[arg(short, long)]
        filter: Option<String>,

        /// Ignore case for the filter
        #[arg(short, long)]
        ignore_case: bool,
    },

    /// Deletes the URLs older than a number of days
    PurgeUrls {
        /// Key of the queue to delete URLs from
        #[arg(short, long)]
        key: Option<String>,

        /// Crawl to delete URLs from
        #[arg(short, long, default_value = DEFAULT_CRAWL)]
        crawl_id: String,

        /// Number of days after which the URLs are deleted
        #[arg(short, long)]
        days: u32,
    },

    /// Exports the URLs as JSON, one per line, in the format taken by put-urls
    #[command(
        long_about = "Exports the URLs as JSON, one per line, in the format taken by \
        put-urls; used to migrate the data to a different backend or version.\n\n\
        The dump covers only the node the client connects to: in a cluster, dump every node \
        and concatenate the files. Deactivate the Frontier and stop the URL injection first \
        if the dump must be complete."
    )]
    DumpUrls {
        /// Output file, compressed with gzip if its name ends in .gz; defaults to the
        /// standard output
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// Restricts the dump to this crawl; by default every crawl is dumped
        #[arg(short, long)]
        crawl_id: Option<String>,

        /// Number of connections dumping the content queue by queue (1 streams each crawl
        /// whole in a single call)
        #[arg(short, long, default_value_t = 1)]
        threads: usize,
    },

    /// Prints the shell completion script for uf
    #[command(long_about = "Prints the shell completion script for uf, which completes \
        the commands, options and the values they accept.\n\n\
        e.g. for bash: uf completions bash > ~/.local/share/bash-completion/completions/uf")]
    Completions {
        /// Shell to complete for
        shell: Shell,
    },
}

#[tokio::main]
async fn main() -> Result<ExitCode, Box<dyn std::error::Error>> {
    let args = Args::parse();

    // the only command which does not need the Frontier
    if let Action::Completions { shell } = args.action {
        generate(shell, &mut Args::command(), "uf", &mut std::io::stdout());
        return Ok(ExitCode::SUCCESS);
    }

    let address = format!("http://{}:{}", args.host, args.port);

    let mut client = UrlFrontierClient::connect(address.clone()).await?;

    match args.action {
        Action::ListNodes => {
            let nodes = client.list_nodes(Request::new(Empty {})).await?;
            for node in nodes.into_inner().values {
                println!("{}", node);
            }
        }

        Action::ListCrawls { local } => {
            let crawls = client.list_crawls(Request::new(Local { local })).await?;
            for crawl in crawls.into_inner().values {
                println!("{}", crawl);
            }
        }

        Action::ListQueues {
            number_queues,
            mut start,
            output,
            include_inactive,
            crawl_id,
            local,
        } => {
            let mut pagination = Pagination {
                start,
                size: number_queues,
                include_inactive,
                crawl_id,
                local,
            };
            match output {
                // only the requested page
                None => {
                    let queues = client.list_queues(Request::new(pagination)).await?;
                    for queue in queues.into_inner().values {
                        println!("{}", queue);
                    }
                }
                // all the queues, page after page
                Some(path) => {
                    let mut out = BufWriter::new(File::create(path)?);
                    loop {
                        pagination.start = start;
                        let queues = client
                            .list_queues(Request::new(pagination.clone()))
                            .await?
                            .into_inner();
                        if queues.values.is_empty() {
                            break;
                        }
                        start += queues.values.len() as u32;
                        for queue in queues.values {
                            writeln!(out, "{}", queue)?;
                        }
                    }
                    out.flush()?;
                }
            }
        }

        Action::ListUrls {
            number_urls,
            start,
            key,
            output,
            crawl_id,
            local,
            json,
            parse_date,
            filter,
            ignore_case,
        } => {
            let params = ListUrlParams {
                start,
                size: number_urls,
                key: key.unwrap_or_default(),
                crawl_id,
                local,
                filter,
                ignore_case: ignore_case.then_some(true),
            };
            let mut out: Box<dyn Write> = match output {
                Some(path) => Box::new(BufWriter::new(File::create(path)?)),
                None => Box::new(std::io::stdout().lock()),
            };
            let mut stream = client.list_ur_ls(Request::new(params)).await?.into_inner();
            while let Some(item) = stream.message().await? {
                if json {
                    writeln!(out, "{}", serde_json::to_string(&item)?)?;
                } else {
                    writeln!(out, "{}", url_and_date(&item, parse_date))?;
                }
            }
            out.flush()?;
        }

        Action::GetStats {
            key,
            crawl_id,
            local,
        } => {
            let response = client
                .get_stats(Request::new(QueueWithinCrawlParams {
                    key: key.unwrap_or_default(),
                    crawl_id,
                    local,
                }))
                .await?;
            let stats = response.into_inner();
            println!("crawl ID:        {}", stats.crawl_id);
            println!("URLs:            {}", stats.size);
            println!("in process:      {}", stats.in_process);
            println!("queues:          {}", stats.number_of_queues);
            // sort the custom counts so that the output is stable
            let mut counts: Vec<_> = stats.counts.iter().collect();
            counts.sort();
            for (name, value) in counts {
                println!("{:<16} {}", format!("{}:", name), value);
            }
        }

        Action::PutUrls {
            file,
            crawl_id,
            threads,
            in_flight,
            batch,
        } => {
            let ok = put_urls::run(address, file, crawl_id, threads, in_flight, batch).await;
            if !ok {
                return Ok(ExitCode::FAILURE);
            }
        }

        Action::GetUrls {
            key,
            delay,
            max_urls,
            max_queues,
            crawl_id,
        } => {
            let item = match crawl_id {
                None => get_params::Item::AnyCrawlId(AnyCrawlId {}),
                Some(crawl_id) => get_params::Item::CrawlId(crawl_id),
            };
            let params = GetParams {
                max_urls_per_queue: max_urls,
                max_queues,
                key: key.unwrap_or_default(),
                delay_requestable: delay,
                item: Some(item),
            };
            let start = Instant::now();
            let mut stream = client.get_ur_ls(Request::new(params)).await?.into_inner();
            while let Some(info) = stream.message().await? {
                println!("{}", serde_json::to_string(&info)?);
            }
            eprintln!("Total time: {} msec", start.elapsed().as_millis());
        }

        Action::SetActive { state, local } => {
            client
                .set_active(Request::new(Active { state, local }))
                .await?;
            println!("Active: {}", state);
        }

        Action::GetActive { local } => {
            let active = client.get_active(Request::new(Local { local })).await?;
            println!("Active: {}", active.into_inner().state);
        }

        Action::DeleteQueue {
            key,
            crawl_id,
            local,
        } => {
            let deleted = client
                .delete_queue(Request::new(QueueWithinCrawlParams {
                    key,
                    crawl_id,
                    local,
                }))
                .await?;
            println!("{} URLs deleted", deleted.into_inner().value);
        }

        Action::DeleteCrawl { crawl_id, local } => {
            let deleted = client
                .delete_crawl(Request::new(DeleteCrawlMessage {
                    value: crawl_id,
                    local,
                }))
                .await?;
            println!("{} URLs deleted", deleted.into_inner().value);
        }

        Action::SetLogLevel {
            package,
            level,
            local,
        } => {
            // the values are restricted by clap to the names of the enum
            let value = Level::from_str_name(&level).unwrap();
            client
                .set_log_level(Request::new(LogLevelParams {
                    package: package.clone(),
                    level: value as i32,
                    local,
                }))
                .await?;
            println!("Set log level for {} to {}", package, level);
        }

        Action::SetCrawlLimit {
            key,
            crawl_id,
            limit,
            local,
        } => {
            client
                .set_crawl_limit(Request::new(CrawlLimitParams {
                    key,
                    limit,
                    crawl_id,
                    local,
                }))
                .await?;
        }

        Action::GetUrlStatus {
            url,
            crawl_id,
            key,
            parse_date,
        } => {
            let request = UrlStatusRequest {
                url: url.clone(),
                key: key.unwrap_or_default(),
                crawl_id,
            };
            match client.get_url_status(Request::new(request)).await {
                Ok(item) => println!("{}", url_and_date(&item.into_inner(), parse_date)),
                Err(status) if status.code() == Code::NotFound => {
                    println!("URL is not in frontier: {}", url);
                    return Ok(ExitCode::FAILURE);
                }
                Err(status) => return Err(status.into()),
            }
        }

        Action::CountUrls {
            key,
            crawl_id,
            local,
            filter,
            ignore_case,
        } => {
            let count = client
                .count_ur_ls(Request::new(CountUrlParams {
                    key: key.unwrap_or_default(),
                    crawl_id,
                    filter,
                    ignore_case: ignore_case.then_some(true),
                    local: Some(local),
                }))
                .await?;
            println!("{} URLs in frontier", count.into_inner().value);
        }

        Action::PurgeUrls {
            key,
            crawl_id,
            days,
        } => {
            let purged = client
                .purge_ur_ls(Request::new(PurgeUrlParams {
                    key: key.unwrap_or_default(),
                    crawl_id,
                    days,
                }))
                .await?;
            println!("Purged {} URLs in frontier", purged.into_inner().value);
        }

        Action::DumpUrls {
            output,
            crawl_id,
            threads,
        } => {
            let ok = dump_urls::run(client, address, output, crawl_id, threads).await;
            if !ok {
                return Ok(ExitCode::FAILURE);
            }
        }

        Action::Completions { .. } => unreachable!("handled before connecting"),
    }

    Ok(ExitCode::SUCCESS)
}

/// Formats a URL item as `url;refetch date`; the date is 0 for a URL which was
/// discovered but never fetched
fn url_and_date(item: &UrlItem, parse_date: bool) -> String {
    let (info, date): (Option<&UrlInfo>, u64) = match &item.item {
        Some(url_item::Item::Known(known)) => (known.info.as_ref(), known.refetchable_from_date),
        Some(url_item::Item::Discovered(discovered)) => (discovered.info.as_ref(), 0),
        None => (None, 0),
    };
    let url = info.map(|info| info.url.as_str()).unwrap_or_default();
    format!("{};{}", url, format_date(date, parse_date))
}

/// Formats a date expressed in seconds since the epoch, in the local time zone if `parse` is set
fn format_date(seconds: u64, parse: bool) -> String {
    if parse {
        if let Some(date) = LocalTime.timestamp_opt(seconds as i64, 0).single() {
            return date.format("%Y-%m-%dT%H:%M:%S").to_string();
        }
    }
    seconds.to_string()
}
