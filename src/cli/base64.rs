use std::{
    fmt::{self},
    str::FromStr,
};

use clap::Parser;

use super::verify_file;

/// CLI子命令枚举，支持base64编码/解码操作
#[derive(Debug, Parser)]
pub enum Base64SubCommand {
    /// base64编码命令，将输入数据编码为base64格式
    #[command(name = "encode", about = "Encode a string to base64")]
    Encode(Base64EncodeOpts),
    /// base64解码命令，将base64数据解码为原始格式
    #[command(name = "decode", about = "Decode a base64 string")]
    Decode(Base64DecodeOpts),
}

/// base64编码命令参数选项
#[derive(Debug, Parser)]
pub struct Base64EncodeOpts {
    /// 输入源文件路径（"-"表示从标准输入读取）
    #[arg(short, long, value_parser = verify_file,default_value = "-")]
    pub input: String,
    /// 指定base64编码格式（standard/urlsafe）
    #[arg(long, value_parser = parse_base64_format, default_value = "standard")]
    pub format: Base64Format,
}

/// base64解码命令参数选项
#[derive(Debug, Parser)]
pub struct Base64DecodeOpts {
    /// 输入源文件路径（"-"表示从标准输入读取）
    #[arg(short, long, value_parser = verify_file,default_value = "-")]
    pub input: String,
    /// 指定base64解码格式（standard/urlsafe）
    #[arg(long, value_parser = parse_base64_format, default_value = "standard")]
    pub format: Base64Format,
}

/// 支持的base64格式类型枚举
#[derive(Debug, Clone, Copy)]
pub enum Base64Format {
    /// 标准base64格式（使用+/作为填充字符）
    Standard,
    /// URL安全的base64格式（使用-_作为填充字符）
    UrlSafe,
}

/// 将字符串解析为Base64Format枚举
/// 参数format: 输入格式字符串（"standard"或"urlsafe"）
/// 返回解析结果或错误信息
fn parse_base64_format(format: &str) -> Result<Base64Format, anyhow::Error> {
    format.parse()
}

// 实现从字符串到Base64Format的转换逻辑
impl FromStr for Base64Format {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "standard" => Ok(Base64Format::Standard),
            "urlsafe" => Ok(Base64Format::UrlSafe),
            _ => Err(anyhow::anyhow!("Invalid base64 format")),
        }
    }
}

// 实现Base64Format到字符串切片的转换，用于命令行参数处理
impl From<Base64Format> for &'static str {
    fn from(format: Base64Format) -> &'static str {
        match format {
            Base64Format::Standard => "standard",
            Base64Format::UrlSafe => "urlsafe",
        }
    }
}

// 为Base64Format实现Display trait，支持格式化输出
impl fmt::Display for Base64Format {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<&'static str>::into(*self))
    }
}
