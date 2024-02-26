use super::{CommandRunner, DownloadArgs};
use anyhow::{anyhow, Result};
use std::fs::{self, File};
use std::io::ErrorKind;
use std::io::{copy, Cursor};
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
        let bytes = reqwest::get(URL).await?.bytes().await?;
        let mut content = Cursor::new(bytes);

        let fname = tmp_dir.path().join("cedict.zip");
        let mut dest = File::create(&fname)?;

        copy(&mut content, &mut dest)?;

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
        let output = Command::new("unzip")
            .arg("-o")
            .arg(fname.to_str().unwrap())
            .arg("-d")
            .arg(format!("{}/cedict.u8", self.args.output_path))
            .output()
            .map_err(|err| anyhow!("Expect to unzip the targeted cedict file {err}"))?;

        dbg!(output);

        Ok(())
    }
}
