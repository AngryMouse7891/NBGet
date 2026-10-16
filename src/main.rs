use clap::Parser;
mod download;
mod messages;
use url::Url;
use percent_encoding::percent_decode_str;


fn get_filename_from_url(url_str: &str) -> Option<String> {
    let url = Url::parse(url_str).ok()?;
    let last_segment = url.path_segments()?.last()?;
    
    if last_segment.is_empty() {
        return None;
    }
    
    // 解码百分号编码，处理非 UTF-8 的容错
    let decoded = percent_decode_str(last_segment)
        .decode_utf8()
        .map(|s| s.into_owned())
        .unwrap_or_else(|_| last_segment.to_string());
    
    Some(decoded)
}

#[derive(Parser, Debug)]
#[command(name="nbget", author, version, about="一个使用Rust语言写的cli下载工具", long_about = None)]
struct Args {
    /// 指定要下载的URL
    url: String,
    #[arg(short, long)]
    out: Option<String>,
    #[arg(long)]
    display_time: Option<bool>,
    /// 自定义请求头，格式: "Key: Value"，可多次使用
    #[arg(short = 'H', long = "header", value_parser = parse_header)]
    headers: Vec<(String, String)>,
}

fn parse_header(s: &str) -> Result<(String, String), String> {
    if let Some(colon_pos) = s.find(':') {
        let key = s[..colon_pos].trim().to_string();
        let value = s[colon_pos + 1..].trim().to_string();
        Ok((key, value))
    } else {
        Err("Header 格式错误，应为 'Key: Value'".to_string())
    }
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let out_path = args.out
        .as_deref()
        .filter(|s| !s.is_empty())
        .map(str::to_string)
        .unwrap_or_else(|| get_filename_from_url(args.url.as_str()).unwrap());
    
    // 简化 display_time 的处理逻辑
    let dt = args.display_time.unwrap_or(false);
    
    download::download_file(
        args.url.as_str(),
        out_path.as_str(),
        dt,
        args.headers
    ).await;
}