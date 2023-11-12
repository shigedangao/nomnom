use anyhow::Result;

mod cmd;
mod hsk;
mod util;
mod progress;

#[tokio::main]
async fn main() -> Result<()> {
    cmd::run().await?;

    Ok(())
}
