pub mod frontier {
    tonic::include_proto!("urlfrontier");
}

use clap::{Parser, Subcommand};
use frontier::url_frontier_client::UrlFrontierClient;
use frontier::QueueWithinCrawlParams;
use tonic::Request;

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
    GetStats,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let mut address = "http://".to_string();
    address.push_str(&args.host);
    address.push_str(":");
    address.push_str(&args.port.to_string());

    let mut client = UrlFrontierClient::connect(address).await?;

    match args.action {
        Action::GetStats => {
            let response = client
                .get_stats(Request::new(QueueWithinCrawlParams {
                    key: String::from(""),
                    crawl_id: String::from(""),
                    local: false,
                }))
                .await?;
            println!("{:?}", response);
        }
    }

    Ok(())
}
