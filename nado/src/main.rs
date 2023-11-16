use anyhow::Result;
use std::process;

mod cmd;
mod hsk;
mod progress;
mod util;

#[tokio::main]
async fn main() -> Result<()> {
    if let Err(err) = cmd::run().await {
        eprintln!("{}", err);
        process::exit(2)
    };

    Ok(())
}
