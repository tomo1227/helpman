#![allow(unused)]
use anyhow::{Context, Result};
use clap::{Parser, Subcommand};


#[derive(Parser)]
struct Cli {
    // #[clap(short, long, help = "オプションのテキスト")]
    // text: Option<String>,
    cmd: String,
}

#[derive(Debug)]
struct CustomError(String);
fn main() -> Result<()> {
    let args = Cli::parse();
    let content = std::fs::read_to_string(["doc/",&args.cmd, ".txt"].concat())
        .with_context(|| format!("エラー内容 : {} が見つかりません", args.cmd))?;

    println!("{}", content);
    // println!("{}", args.text());
    Ok(())
}
