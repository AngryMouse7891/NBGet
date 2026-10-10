use colored::Colorize;
use file_retriever::{self, RetrieverBuilder};
use reqwest::Client;
use tokio::fs::OpenOptions;
use std::time::Instant;

pub async fn download_file(url: &str,out: &str,print_time: bool) {
    let start = Instant::now();
    let retriever = RetrieverBuilder::new()
        .show_progress(true)
        .workers(1)
        .build();

    let request = Client::new()
        .get(url)
        .build()
        .unwrap();

    let output_file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(out)
        .await
        .unwrap();

    let result = retriever.download_file(request, output_file).await;
    let duration = start.elapsed();
    if print_time {
        println!("🕒 {:.1}s",duration.as_secs_f64().to_string().as_str().blue());
    }
}