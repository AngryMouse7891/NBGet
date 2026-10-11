use clap::Parser;
mod download;
use url::Url;

fn get_filename_from_url(url_str: &str) -> Option<String> {
    let url = Url::parse(url_str).ok()?;
    let segments = url.path_segments()?;
    // 关键修复：使用 ? 解包 Option，如果为 None 会自动返回 None
    let last = segments.last()?;

    if last.is_empty() {
        return None;
    }

    // 解码百分号编码（如 %20 -> 空格）
    Some(percent_decode(last))
}

// 简单的百分号解码（或直接用 percent-encoding crate）
fn percent_decode(input: &str) -> String {
    // 实际项目中建议用 percent_encoding crate
    input.replace("%20", " ") // 修复：合并到一行
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