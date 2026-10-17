<p align="center"> 
   <img src="doc/img/cover.png" alt="封面" width="60%" height="auto"/> 
</p> 

# <p align="center"> NBGet </p>

<p align="center">
	<img src="https://gitee.com/AngryMouse7891/NBGet/badge/star.svg?theme=dark" alt="star" /> 
	<img src="https://gitee.com/AngryMouse7891/NBGet/badge/fork.svg?theme=dark" alt="star" />
   <img src="https://img.shields.io/badge/gitee-repo-blue?logo=gitee" alt="Gitee Repo">
   <img src="https://img.shields.io/badge/dynamic/json?url=https%3A%2F%2Fgitee.com%2Fapi%2Fv5%2Frepos%2FAngryMouse7891%2FNBGet%2Flicense&query=%24.license&label=license&color=green
   " alt="License">
   <img src="https://img.shields.io/badge/dynamic/toml?url=https%3A%2F%2Fraw.giteeusercontent.com%2FAngryMouse7891%2FNBGet%2Fraw%2Fmain%2FCargo.toml&query=%24.package.version&label=latest&color=orange
" alt="Version"/>
</p>

## 简介

NBGet是一个使用了Rust语言+tokio+reqwest+file-retriever的文件下载CLI工具。

关于名字的起源的话，“牛逼”的拼音首字母是`NB`，故得名NBGet。

> [!NOTE]
> 这是我的第一个Rust项目，本人可能对Rust语言不太了解，不喜勿喷

## 特性

* ✅ ⭐完全使用Rust编写
* ✅ ⚡轻量且性能极高
* ✅ 🕒支持显示下载耗时
* ✅ 💻完全跨平台，支持Windows和Linux

## 使用方法

### 命令帮助

```bash
一个使用Rust语言开发的可以下载文件的CLI工具，就是牛逼！

Usage: nbget.exe [OPTIONS] <URL>

Arguments:
  <URL>  指定要下载的URL

Options:
  -o, --out <OUT>                输出文件名
  -T, --timeout <TIMEOUT>        设置超时 [default: 15]
      --display-time             设置是否显示时间
      --no-progress              设置是否禁用进度条
      --no-clobber               不要下载已存在将被覆盖的文件
      --nologo                   设置是否禁用显示ASCII艺术字和版本等信息
  -H, --header <HEADERS>         自定义请求头，格式: "Key: Value"，可多次使用
  -A, --user-agent <USER_AGENT>  指定下载UA [default: NBGet/0.0.1]
  -h, --help                     Print help
  -V, --version                  Print version

```

### 使用录屏

<video width="500" height="300" controls>
  <source src="data/sample.mp4" type="video/mp4">
</video>

## 速度对比

> [!NOTE]
> 速度仅供参考，请以实际数据为准 <br>
> 每项跑一次，不取平均

### 软件环境

| 软件                                         | 版本                  |
| ------------------------------------------ | ------------------- |
| NBGet                                      | 0.0.1               |
| [Wget](https://www.gnu.org/software/wget/) | 1.25.0              |
| [cURL](https://curl.se/)                   | 8.21.0              |
| [Aria2](https://aria2.github.io/)          | 1.37.0 (2026/04/30) |

### 速度（默认单线程）

| 文件                                                                                                                                                      | 大小        | NBGet        | cURL     | Wget         | Aria2        |
| ------------------------------------------------------------------------------------------------------------------------------------------------------- | --------- | ------------ | -------- | ------------ | ------------ |
| [Git 2.56.0.2](https://mirrors.tuna.tsinghua.edu.cn/github-release/git-for-windows/git/Git%20for%20Windows%20v2.56.0.windows.2/Git-2.56.0.2-64-bit.exe) | 63.67 MiB | 6.991s (4th) | 🥈6.272s | 🥇6.189s     | 🥉6.498s     |
| [小熊猫Dev-C++ v6.7.5 （无编译器）](https://gitee.com/royqh1979/Dev-CPP/releases/download/v6.7.5/Dev-Cpp.6.7.5.No.Compiler.Setup.exe)                            | 2.65 MiB  | 4.604s (4th) | 🥈2.844s | 🥇2.836s     | 🥉4.203s     |
| [AtomCode v5.2.3](https://gitcode.com/atomgit_atomcode/atomcode)                                                                                        | 41.78 MiB | 🥉4.232s     | 🥇4.180s | 5.061s (4th) | 🥈4.230s     |
| [Node.js v26.12.0](https://registry.npmmirror.com/-/binary/node/v26.12.0/node-v26.12.0-x64.msi)                                                         | 35.24 MiB | 🥉3.836s     | 🥇3.721s | 🥈3.733s     | 4.972s (4th) |
| 最快次数                                                                                                                                                    |           | 0            | 2        | 2            | 0            |
| 完成次数                                                                                                                                                    |           | 4/4          | 4/4      | 4/4          | 4/4          |

## 使用库

| 包名 | 版本 | 描述 | 类型 |
| :---------: | :-------: | :----------: | :----------: |
| [`clap`](https://crates.io/crates/clap) | `4.5.42` | 命令行参数解析 | 一般依赖 |
| [`file-retriever`](https://crates.io/crates/file-retriever) | `0.1.7` | 支持进度显示的文件下载库 | 一般依赖 |
| [`reqwest`](https://crates.io/crates/reqwest) | `0.12.13` | HTTP客户端 | 一般依赖 |
| [`tokio`](https://crates.io/crates/tokio) | `1.47.1` | 异步运行时 | 一般依赖 |
| [`url`](https://crates.io/crates/url) | `2.5.8` | 用于处理URL的标准库 | 一般依赖 |
| [`colored`](https://crates.io/crates/colored) | `3.1.1` | 彩色输出库 | 一般依赖 |
| [`percent-encoding`](https://crates.io/crates/percent-encoding) | `2.3.2` | 百分比编解码 | 一般依赖 |
| [`chrono`](https://crates.io/crates/chrono) | `0.4.45` | 日期时间库 | 一般依赖 |
| [`embed-resource`](https://crates.io/crates/embed-resource) | `3.0.12` | 处理Windows资源的编译和包含 | 构建依赖 |

## 开源协议

该项目使用[GNU 宽通用公共许可证 v3.0](https://www.gnu.org/licenses/lgpl-3.0.zh-cn.html)开源。​
