use anyhow::Result;
use async_trait::async_trait;
use clap::{Parser, ValueEnum};

mod generate;

#[async_trait]
trait CommandRunner {
    async fn run(args: &CliArgs) -> Result<()>;
}

#[derive(Parser)]
#[command(name = "nomnom")]
enum Command {
    Generate(CliArgs),
}

#[derive(Debug, ValueEnum, Clone)]
enum OutputFormat {
    Json,
    Csv,
}

#[derive(clap::Args)]
#[command(
    author = "shigedangao",
    version = "0.3.0",
    about = "generate cedict.u8 into your desired format output",
    long_about = None
)]
struct CliArgs {
    #[clap(short = 'e', long, value_parser)]
    file_path: String,

    #[clap(short, long, value_parser)]
    output_path: String,

    #[clap(short = 'f', long, value_parser)]
    output_format: OutputFormat,
}

/// Run the command
pub async fn run() -> Result<()> {
    let cmd = Command::parse();

    match cmd {
        Command::Generate(args) => generate::Gen::run(&args).await?,
    };

    Ok(())
}
