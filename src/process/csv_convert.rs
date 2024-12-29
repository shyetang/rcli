use crate::opts::OutputFormat;
use anyhow::Result;
use csv::Reader;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs;

/// 表示球员的信息，包括姓名、位置、出生日期、国籍和球衣号码。
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct Player {
    // #[serde(rename = "Name")]
    name: String,
    // #[serde(rename = "Position")]
    position: String,
    #[serde(rename = "DOB")]
    dob: String,
    // #[serde(rename = "Nationality")]
    nationality: String,
    #[serde(rename = "Kit Number")]
    kit: u8,
}

/// 处理CSV文件，将其内容转换为JSON格式并写入输出文件。
///
/// # 参数
/// * `input` - 输入CSV文件的路径。
/// * `output` - 输出JSON文件的路径。
///
/// # 返回值
/// * `Result<()>` - 如果操作成功，返回Ok(())；否则返回错误。
pub fn process_csv(input: &str, output: String, format: OutputFormat) -> Result<()> {
    // 从输入文件路径创建CSV读取器
    let mut reader = Reader::from_path(input)?;
    // 预分配内存以提高性能
    let mut ret = Vec::with_capacity(128);
    // 获取CSV文件的表头
    let headers = reader.headers()?.clone();
    // 遍历CSV文件中的每一行记录
    for result in reader.records() {
        // 解析一行记录
        let record = result?;
        // 将表头与记录数据组合成JSON对象
        // headers.iter() -> 使用 headers 的迭代器
        // record.iter()-> 使用 record 的迭代器
        // zip() -> 将两个迭代器合并为一个元组的迭代器［(header,record),..］
        // collect::<Value>()-> 将元组的迭代器转换为 JSON Value
        let json_value = headers.iter().zip(record.iter()).collect::<Value>();
        // 将创建的JSON对象添加到结果向量中
        ret.push(json_value);
    }
    let content = match format {
        OutputFormat::Json => serde_json::to_string_pretty(&ret)?,
        OutputFormat::Yaml => serde_yaml::to_string(&ret)?,
    };
    // 将数据写入输出文件
    // => () 返回一个元组
    fs::write(output, content)?;
    // 返回Ok表示函数成功完成
    Ok(())
}
