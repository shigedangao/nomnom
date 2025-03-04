use super::{HSKLevel, Source};
use anyhow::{Result, anyhow};
use scraper::html::Html;
use scraper::selector::Selector;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::task::JoinHandle;

pub const URLS: [&str; 6] = [
    "https://mandarinbean.com/new-hsk-1-word-list/",
    "https://mandarinbean.com/new-hsk-2-word-list/",
    "https://mandarinbean.com/new-hsk-3-word-list/",
    "https://mandarinbean.com/new-hsk-4-word-list/",
    "https://mandarinbean.com/new-hsk-5-word-list/",
    "https://mandarinbean.com/new-hsk-6-word-list/",
];

#[derive(Debug)]
pub struct Mandarinbean {
    contents: Vec<String>,
}

impl Source for Mandarinbean {
    async fn get_contents(self) -> Result<HashMap<String, HSKLevel>> {
        let mut tasks = Vec::new();
        let hsk_items = Arc::new(Mutex::new(HashMap::new()));

        for (idx, content) in self.contents.into_iter().enumerate() {
            let handle = hsk_items.clone();

            let handle: JoinHandle<Result<()>> = tokio::spawn(async move {
                let document = Html::parse_document(&content);
                let selector =
                    Selector::parse("tr").map_err(|_| anyhow!("Unable to find the selector tr"))?;

                let items = document.select(&selector);

                for item in items {
                    let td_selector = Selector::parse("td").unwrap();
                    let mut td_items = item.select(&td_selector);
                    let cn_char = td_items.nth(1);

                    if let Some(c) = cn_char {
                        let item_str = c.text().collect::<String>().trim().to_owned();

                        if let Ok(mut h) = handle.lock() {
                            h.insert(item_str, HSKLevel::from(idx));
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
            Err(_) => Err(anyhow!("Unable to consume the inner wrapper")),
        }
    }

    fn set_contents(contents: Vec<String>) -> Self {
        Self { contents }
    }
}
