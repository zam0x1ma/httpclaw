use clap::Parser;
use reqwest::Client;
use reqwest::Method;
use reqwest::header;
use std::io::prelude::*;
use std::io;

/// Take a list of domains and probe for working http and https servers
#[derive(Parser, Debug)]
#[clap(about)]
struct Args {
    /// Set the concurrency level (split equally between HTTPS and HTTP requests)
    #[clap(short, long, value_parser, default_value_t = 20)]
    concurrency: u32,

    /// Add additional probe (protocol:port)
    #[clap(short, long, value_parser)]
    probes: Vec<String>,

    /// Skip the default probes (http:80 and https:443)
    #[clap(short, long, value_parser, default_value_t = false)]
    skip_default: bool,

    /// Timeout (miliseconds)
    #[clap(short, long, value_parser, default_value_t = 10000)]
    timeout: u32,

    /// Only try plain HTTP if HTTPS fails
    #[clap(long, value_parser, default_value_t = false)]
    prefer_https: bool,

    /// HTTP method to use
    #[clap(short, long, value_parser, default_value = "GET")]
    method: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let stdin = io::stdin();
    let client = Client::new();
    for line in stdin.lock().lines() {
        let domain = line.unwrap();
        let http_url = format!("http://{domain}");
        let https_url = format!("https://{domain}");

        if is_listening(&client, &http_url, Method::GET).await {
            println!("{http_url}");
        }

        if is_listening(&client, &https_url, Method::GET).await {
            println!("{https_url}");
        }
    }
}

async fn is_listening(client: &reqwest::Client, url: &String, method: reqwest::Method) -> bool {
    match client.request(method, url)
        .header(header::CONNECTION, "Close")
        .send()
        .await {
            Ok(_) => true,
            Err(_) => false
        }
}
