use clap::{Parser, ValueEnum};
use std::fs;

mod cedict;
mod error;
mod hsk;
mod util;

#[derive(Debug, ValueEnum, Clone)]
enum OutputFormat {
    Csv,
    Json
}

#[derive(Parser)]
#[command(name = "nomnom")]
#[command(bin_name = "nomnom")]
enum Generator {
    Generate(CliArgs)
}

#[derive(clap::Args)]
#[command(author = "shigedangao", version = "0.1.2", about = "parsing cedict.u8 to csv")]
struct CliArgs {
    #[clap(short, long, value_parser)]
    cedict_path: String,

    #[clap(short, long, value_parser)]
    zh_hsk_path: String,

    #[clap(short, long, value_parser)]
    output_path: String,

    #[clap(short, long, value_parser)]
    format: OutputFormat
}

fn main() {
    let Generator::Generate(args) = Generator::parse();

    // load the hsk file which will be used to add to the level.
    let hsk = match hsk::from_csv(&args.zh_hsk_path) {
        Ok(v) => v,
        Err(err) => {
            println!("Unable to load the hsk dictionary: {err}");
            return;
        }
    };

    let items = cedict::Cedict::parse(&args.cedict_path, hsk);
    if let Err(err) = items {
        println!("Unable to parse the cc-cedict dictionary: {err}");
        return;
    }

    let items = items.unwrap();
    let output = match args.format {
        OutputFormat::Json => util::to_json(items),
        OutputFormat::Csv => util::to_csv(items)
    };

    match output {
        Ok(contents) => {
            if let Err(err) = fs::write(args.output_path, contents) {
                println!("An error ocurred while writing the content to the output {err}")
            }
        },
        Err(err) => println!("Unable to convert the cc-cedict for the targeted format: {err}")
    }

    println!("Generation done ✅")
}
