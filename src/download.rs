use file_retriever::{self, RetrieverBuilder};
use reqwest::Client;
use tokio::fs::OpenOptions;
use std::time::Instant;
use crate::messages::info_msg;
use chrono::Local;

pub async fn download_file(url: &str, out: &str, print_time: bool, headers: Vec<(String, String)>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let now = Local::now();
    info_msg(format!("开始下载：{} （{}）",url,now).as_str());
    let start = Instant::now();
    let retriever = RetrieverBuilder::new()
        .show_progress(true)
        .workers(1)
        .build();

    let mut request_builder = Client::new()
        .get(url);

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
    let duration = start.elapsed();
    if print_time {
        info_msg(format!("✅ 下载完成，耗时 {:.1}s",duration.as_secs_f64().to_string().as_str()).as_str());
    }

    result
}
