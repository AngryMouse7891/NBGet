use clap::Parser;
mod download;
use url::Url; // cargo add url

fn get_filename_from_url(url_str: &str) -> Option<String> {
    let url = Url::parse(url_str).ok()?;
    let segments = url.path_segments()?;
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
    input
.replace("%20", " ") // 简化示例
}

#[derive(Parser, Debug)]
#[command(name="nbget", author, version, about="一个使用Rust语言写的cli下载工具", long_about = None)]
struct Args {
    /// 指定要下载的URL
    url: String,
    #[arg(short, long)]
    out: Option<String>,
    #[arg(long)]
    display_time: Option<bool>
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let out_path = args.out
    .as_deref()
    .filter(|s| !s.is_empty())
    .map(str::to_string)
    .unwrap_or_else(|| get_filename_from_url(args.url.as_str()).unwrap());
    let dt;
    if args.display_time.is_none() {
        dt=false;
    }else {
        dt=args.display_time.unwrap();
    }
    download::download_file(
        args.url.as_str(), 
        out_path.as_str(),
        dt)
    .await;
}
