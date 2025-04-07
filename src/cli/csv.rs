use super::verify_file;
use clap::Parser;
use std::fmt;
use std::str::FromStr;

// 定义输出格式的枚举，用于表示数据的序列化格式
#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    Json, // JSON 格式
    Yaml, // YAML 格式
}

// 定义CSV处理的命令行选项结构体
#[derive(Debug, Parser)]
pub struct CsvOpts {
    // 输入文件路径，通过自定义解析器验证文件存在性
    #[arg(short,long,value_parser=verify_file)]
    pub input: String,

    // 输出文件路径，可选，如果不提供，则默认输出到控制台
    #[arg(short, long)]
    pub output: Option<String>,

    // 输出格式，通过自定义解析器解析， 默认为 JSON
    #[arg(long, value_parser = parse_format, default_value = "json")]
    pub format: OutputFormat,

    // CSV 文件的分隔符，默认为逗号
    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,

    // 是否包含表头，默认为 true
    #[arg(long, default_value_t = true)]
    pub header: bool,
}

// 自定义解析器，将字符串解析为OutputFormat枚举
fn parse_format(format: &str) -> Result<OutputFormat, anyhow::Error> {
    format.parse::<OutputFormat>()
}

// 实现从OutputFormat枚举到字符串的转换
impl From<OutputFormat> for &'static str {
    fn from(format: OutputFormat) -> Self {
        match format {
            OutputFormat::Json => "json",
            OutputFormat::Yaml => "yaml",
        }
    }
}

// 实现从字符串到OutputFormat枚举的转换
impl FromStr for OutputFormat {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "json" => Ok(OutputFormat::Json),
            "yaml" => Ok(OutputFormat::Yaml),
            _ => Err(anyhow::anyhow!("Invalid format")),
        }
    }
}

// 实现OutputFormat枚举的显示格式化
impl fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}
