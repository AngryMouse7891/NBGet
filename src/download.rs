use file_retriever::{self, RetrieverBuilder};
use reqwest::Client;
use tokio::fs::OpenOptions;
use std::time::{Duration, Instant};
use crate::messages::info_msg;
use chrono::Local;

pub async fn download_file(url: &str, out: &str, timeout: u64, headers: Vec<(String, String)>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let now = Local::now();
    info_msg(format!("开始下载：{} （{}）",url,now).as_str());
    let retriever = RetrieverBuilder::new()
        .show_progress(true)
        .workers(1)
        .build();

    let mut request_builder = Client::new()
        .get(url)
        .timeout(Duration::from_secs(timeout));

    // 添加自定义请求头
    for (key, value) in headers {
        request_builder = request_builder.header(key, value);
    }

    let request = request_builder.build().unwrap();

    let output_file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(out)
        .await
        .unwrap();

    let result = retriever.download_file(request, output_file).await;

    result
}
