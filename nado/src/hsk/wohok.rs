use super::{HSKLevel, Source};
use anyhow::{anyhow, Result};
use scraper::html::Html;
use scraper::selector::Selector;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::task::JoinHandle;

pub const URLS: [&str; 1] = ["https://wohok.com/hsk-7-9-vocabulary-list-for-hsk-3-0/"];

#[derive(Debug)]
pub struct Wohok {
    contents: Vec<String>,
}

impl Source for Wohok {
    async fn get_contents(self) -> Result<HashMap<String, HSKLevel>> {
        let mut tasks = Vec::new();
        let hsk_items = Arc::new(Mutex::new(HashMap::new()));

        for content in self.contents.into_iter() {
            let handle = hsk_items.clone();

            let handle: JoinHandle<Result<()>> = tokio::spawn(async move {
                let document = Html::parse_document(&content);
                let selector =
                    Selector::parse("tr").map_err(|_| anyhow!("Unable to find the selector tr"))?;

                let items = document.select(&selector);

                for item in items {
                    let td_selector = Selector::parse("td").unwrap();
                    let mut td_items = item.select(&td_selector);
                    let cn_char = td_items.nth(0);

                    if let Some(c) = cn_char {
                        let item_str = c.text().collect::<String>().trim().to_owned();

                        if let Ok(mut h) = handle.lock() {
                            h.insert(item_str, HSKLevel::HSK7);
                        }
                    }
                }

                Ok(())
            });

            tasks.push(handle);
        }

        for task in tasks {
            task.await??;
        }

        match Arc::try_unwrap(hsk_items) {
            Ok(mutex) => Ok(mutex.into_inner()?),
            Err(_) => return Err(anyhow!("Unable to consume the inner wrapper")),
        }
    }

    fn set_contents(contents: Vec<String>) -> Self {
        Self { contents }
    }
}
