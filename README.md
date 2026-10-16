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

## 使用库

| 包名 | 版本 | 描述 |
| :---------: | :-------: | :----------: |
| [`clap`](https://crates.io/crates/clap) | `4.5.42` | 命令行参数解析 |
| [`file-retriever`](https://crates.io/crates/file-retriever) | `0.1.7` | 支持进度显示的文件下载库 |
| [`reqwest`](https://crates.io/crates/reqwest) | `0.12.13` | HTTP客户端 |
| [`tokio`](https://crates.io/crates/tokio) | `1.47.1` | 异步运行时 |
| [`url`](https://crates.io/crates/url) | `2.5.8` | 用于处理URL的标准库 |
| [`colored`](https://crates.io/crates/colored) | `3.1.1` | 彩色输出库 |
| [`percent-encoding`](https://crates.io/crates/percent-encoding) | `2.3.2` | 百分比编解码 |
| [`chrono`](https://crates.io/crates/chrono) | `0.4.45` | 日期时间库 |

## 开源协议

该项目使用[GNU 宽通用公共许可证 v3.0](https://www.gnu.org/licenses/lgpl-3.0.zh-cn.html)开源。​
