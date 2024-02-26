use anyhow::Result;
use clap::{Parser, ValueEnum};

mod download;
mod generate;

/// Command Runner run the command on the given Arguments
trait CommandRunner {
    async fn run(&self) -> Result<()>;
}

#[derive(clap::Args)]
#[command(
    author = "shigedangao",
    version = "0.3.0",
    about = "generate cedict.u8 into your desired format output",
    long_about = None
)]
#[derive(Debug)]
struct GenerateArgs {
    #[clap(short = 'e', long, value_parser)]
    file_path: String,

    #[clap(short, long, value_parser)]
    output_path: String,

    #[clap(short = 'f', long, value_parser)]
    output_format: OutputFormat,
}

#[derive(clap::Args)]
#[command(
    author = "shigedangao",
    version = "0.3.0",
    about = "download cedict.u8 file",
    long_about = None
)]
#[derive(Debug)]
struct DownloadArgs {
    #[clap(long, value_parser)]
    output_path: String,
}

#[derive(Parser)]
#[command(name = "nomnom")]
enum Command {
    Generate(GenerateArgs),
    Download(DownloadArgs),
}

#[derive(Debug, ValueEnum, Clone)]
enum OutputFormat {
    Json,
    Csv,
}

/// Run the command
pub async fn run() -> Result<()> {
    let cmd = Command::parse();

    match cmd {
        Command::Generate(args) => generate::Gen::new(args).run().await?,
        Command::Download(args) => download::Downloader::new(args).run().await?,
    };

    Ok(())
}
