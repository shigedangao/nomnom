use crate::error::Error;
use crate::log::Logger;
use cedict::parse_cedict_file;
use clap::{Parser, ValueEnum};
use std::fs;

mod cedict;
mod error;
mod hsk;
mod log;
mod pinyin;
mod util;
mod zhuyin;

#[derive(Debug, ValueEnum, Clone)]
enum OutputFormat {
    Csv,
    Json,
}

#[derive(Parser)]
#[command(name = "nomnom")]
#[command(bin_name = "nomnom")]
enum Generator {
    Generate(CliArgs),
}

#[derive(clap::Args)]
#[command(
    author = "shigedangao",
    version = "0.2.1",
    about = "parsing cedict.u8 to csv"
)]
struct CliArgs {
    #[clap(short, long, value_parser)]
    cedict_path: String,

    #[clap(short, long, value_parser)]
    zh_hsk_path: String,

    #[clap(short, long, value_parser)]
    output_path: String,

    #[clap(short, long, value_parser)]
    format: OutputFormat,
}

fn main() {
    let Generator::Generate(args) = Generator::parse();

    // load the hsk file which will be used to add to the level.
    let items = match hsk::from_csv(&args.zh_hsk_path)
        // parse the cedict file with the hsk
        .and_then(|res| parse_cedict_file(&args.cedict_path, res))
    {
        Ok(res) => res,
        Err(err) => return Logger::error("Unable to process files", err),
    };

    let output = match args.format {
        OutputFormat::Json => util::to_json(items),
        OutputFormat::Csv => util::to_csv(items),
    };

    if let Err(err) = output.and_then(|c| fs::write(args.output_path, c).map_err(Error::from)) {
        return Logger::error(
            "Unable to convert the cc-cedict for the targeted format",
            err,
        );
    }

    Logger::success("Generation done");
}
