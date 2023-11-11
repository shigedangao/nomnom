use anyhow::Result;

mod cmd;
mod hsk;
mod util;

#[tokio::main]
async fn main() -> Result<()> {
    cmd::run().await?;

    Ok(())
}
