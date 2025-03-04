use super::{CommandRunner, DownloadArgs};
use crate::progress::ProgressBuilder;
use anyhow::{Result, anyhow};
use futures_util::StreamExt;
use std::fs::{self, File};
use std::io::{ErrorKind, Write};
use std::process::Command;
use tempfile::Builder;

// Constant
const URL: &str = "https://www.mdbg.net/chinese/export/cedict/cedict_1_0_ts_utf-8_mdbg.zip";
const CEDICT: &str = "cedict";

pub struct Downloader {
    args: DownloadArgs,
}

impl Downloader {
    pub fn new(args: DownloadArgs) -> Self {
        Downloader { args }
    }
}

impl CommandRunner for Downloader {
    async fn run(&self) -> Result<()> {
        let tmp_dir = Builder::new().prefix(CEDICT).tempdir()?;
        let url = match &self.args.download_link {
            Some(url) => url.clone(),
            None => URL.to_owned(),
        };

        println!("⚙️ - Downloading cedict.zip ...");
        let request = reqwest::get(url).await?;
        let total_length = request.content_length().unwrap_or_default();

        let mut streams = request.bytes_stream();
        let fname = tmp_dir.path().join("cedict.zip");
        let mut dest = File::create(&fname)?;

        // Initialize progress bar
        let mut pb = ProgressBuilder::new(total_length);

        while let Some(chunk) = streams.next().await {
            let chunk = chunk?;
            dest.write_all(&chunk)?;
            pb.inc(chunk.len() as u64);
        }

        // Check whether the targeted path exist
        fs::create_dir_all(&self.args.output_path)
            .map_err(|err| {
                if let ErrorKind::AlreadyExists = err.kind() {
                    return Ok(());
                }

                Err(err)
            })
            .map_err(|err| anyhow!("Unable to create the directory {:?}", err))?;

        // Unzip the file to the targeted path
        #[cfg(any(target_os = "macos", target_os = "linux"))]
        let output = Command::new("unzip")
            .arg("-j")
            .arg("-o")
            .arg(fname.to_str().unwrap())
            .arg("-d")
            .arg(&self.args.output_path)
            .output()
            .map_err(|err| anyhow!("Expect to unzip the targeted cedict file {err}"))?;

        #[cfg(target_os = "windows")]
        let output = Command::new("expand")
            .arg(fname.to_str().unwrap())
            .arg(&self.args.output_path)
            .output()
            .map_err(|err| anyhow!("Expect to unzip the targeted cedict file {err}"))?;

        match output.status.success() {
            true => println!("📚 - Dictionary has been downloaded"),
            false => println!("📚❌ - Dictionary could not be download"),
        }

        pb.clear();

        Ok(())
    }
}
