use clap::Parser;
mod download;
mod messages;
use url::Url;
use percent_encoding::percent_decode_str;
use std::process::exit;
use std::time::Instant;
use std::path::Path;

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
#[command(name="nbget", author, version, about, long_about = None)]
struct Args {
    /// 指定要下载的URL
    url: String,
    /// 输出文件名
    #[arg(short, long)]
    out: Option<String>,
    /// 设置超时
    #[arg(short='T', long, default_value_t = 15)]
    timeout: u64,
    /// 设置是否显示时间
    #[arg(long, default_value_t = false)]
    display_time: bool,
    /// 设置是否禁用进度条
    #[arg(long, default_value_t = false)]
    no_progress: bool,
    /// 自定义请求头，格式: "Key: Value"，可多次使用
    #[arg(short = 'H', long = "header", value_parser = parse_header)]
    headers: Vec<(String, String)>,
    /// 指定下载UA
    #[arg(short='A', long, default_value_t = format!("NBGet/{}",env!("CARGO_PKG_VERSION")))]
    user_agent: String,
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
    
    let mut outfile=String::from(out_path.as_str());

    if Path::new(outfile.as_str()).exists() {
        messages::warn_msg(format!("文件已存在，将会命名为{}……",outfile).as_str());
        outfile+="_";
    }
    
    messages::info_msg2(format!("正在保存至{}……",outfile).as_str());

    let start = Instant::now();
    let download_result=download::download_file(
        args.url.as_str(),
        outfile.as_str(),
        args.timeout,
        args.headers,
        args.user_agent.as_str(),
        args.no_progress
    ).await;

    match download_result {
        Ok(_v) => {
            let duration = start.elapsed();
            if args.display_time {
                messages::info_msg(format!("✅ 下载完成，耗时 {:.1}s",duration.as_secs_f64().to_string().as_str()).as_str());
            }
            else {
                messages::info_msg("✅ 下载完成");
            }
        }
        Err(e) => {
            messages::err_msg(format!("下载失败！原因：{}",e).as_str());
            exit(1);
        }
    }
}