use reqwest::Client;
use reqwest::Method;
use reqwest::header;
use std::io::prelude::*;
use std::io;

#[tokio::main]
async fn main() {
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
