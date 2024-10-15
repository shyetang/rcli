use clap::Parser;
use std::path::Path;

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
}
#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short,long,value_parser=verify_input_file)]
    // 通过value_parser 我们可以预先处理我们的输入
    pub input: String,
    #[arg(short, long, default_value = "output.json")]
    pub output: String,
    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,
    #[arg(long, default_value_t = true)]
    pub header: bool,
}

fn verify_input_file(filename: &str) -> Result<String, &'static str> {
    if Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("File does not exist")
    }
}
