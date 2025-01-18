mod base64;
mod csv;
mod genpass;
mod http;
mod text;

use anyhow::Result;
use std::path::{Path, PathBuf};

pub use self::{
    base64::Base64Format, base64::Base64SubCommand, csv::OutputFormat, http::HttpSubCommand,
    text::TextSignFormat, text::TextSubCommand,
};

use crate::cli::csv::CsvOpts;
use crate::cli::genpass::GenPassOpts;
use clap::Parser;
#[derive(Debug, Parser)]
#[command(name="rcli",version,author,about,long_about=None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand, // 子命令
}
#[derive(Debug, Parser)]
pub enum SubCommand {
    // 定义子命令
    #[command(name = "csv", about = "Show CSV,or convert csv to other formats")]
    Csv(CsvOpts), // csv操作
    #[command(name = "genpass", about = "Generate random password")]
    GenPass(GenPassOpts), // 生成密码
    #[command(subcommand)]
    Base64(Base64SubCommand),
    #[command(subcommand)]
    Text(TextSubCommand),
    #[command(subcommand)]
    Http(HttpSubCommand),
}

fn verify_file(filename: &str) -> Result<String, &'static str> {
    // if input is "-" or file exists
    if filename == "-" || std::path::Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("File does not exist")
    }
}

fn verify_path(path: &str) -> Result<PathBuf, &'static str> {
    let p = Path::new(path);
    if p.exists() && p.is_dir() {
        Ok(path.into())
    } else {
        Err("Path does not exist or is not a directory")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_input_file() {
        assert_eq!(verify_file("-"), Ok("-".into()));
        assert_eq!(verify_file("*"), Err("File does not exist"));
        assert_eq!(verify_file("Cargo.toml"), Ok("Cargo.toml".into()));
        assert_eq!(verify_file("not-exist"), Err("File does not exist"));
    }
}
