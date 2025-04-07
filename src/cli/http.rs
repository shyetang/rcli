use super::verify_path;
use clap::Parser;
use std::path::PathBuf;

// HTTP子命令枚举，用于处理HTTP相关的命令行操作
#[derive(Debug, Parser)]
pub enum HttpSubCommand {
    // Serve子命令，用于启动HTTP文件服务器
    #[command(about = "Serve a directory over HTTP")]
    Serve(HttpServeOpts),
}

// HTTP服务器选项结构体，定义服务器的配置参数
#[derive(Debug, Parser)]
pub struct HttpServeOpts {
    // 服务目录路径，使用-d或--dir指定，默认为当前目录
    #[arg(short, long, value_parser = verify_path, default_value = ".")]
    pub dir: PathBuf,
    // 服务端口号，使用-p或--port指定，默认为8080
    #[arg(short, long, default_value_t = 8080)]
    pub port: u16,
}
