use anyhow::Result;
use mandarinbean::{Mandarinbean, URLS};
use serde::Serialize;
use std::{collections::HashMap, fmt::Display};
use tokio::task::JoinHandle;
use wohok::{Wohok, URLS as WohokURLS};

mod mandarinbean;
mod wohok;

#[derive(Debug, Clone, Serialize)]
pub enum HSKLevel {
    HSK1,
    HSK2,
    HSK3,
    HSK4,
    HSK5,
    HSK6,
    HSK7,
}

impl From<usize> for HSKLevel {
    fn from(value: usize) -> Self {
        match value {
            0 => Self::HSK1,
            1 => Self::HSK2,
            2 => Self::HSK3,
            3 => Self::HSK4,
            4 => Self::HSK5,
            5 => Self::HSK6,
            _ => Self::HSK1,
        }
    }
}

impl Display for HSKLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::HSK1 => write!(f, "hsk1"),
            Self::HSK2 => write!(f, "hsk2"),
            Self::HSK3 => write!(f, "hsk3"),
            Self::HSK4 => write!(f, "hsk4"),
            Self::HSK5 => write!(f, "hsk5"),
            Self::HSK6 => write!(f, "hsk6"),
            Self::HSK7 => write!(f, "hsk7"),
        }
    }
}

pub trait Source {
    /// Download the contents asynchronously based on the given list of URL endpoints
    ///
    /// # Arguments
    ///
    /// * `endpoints` - Vec<String>
    async fn download_contents<S>(endpoints: Vec<S>) -> Result<Self>
    where
        Self: Sized,
        S: AsRef<str> + std::marker::Send,
    {
        let mut tasks = Vec::new();
        let mut html_content: Vec<_> = Vec::new();

        for url in endpoints.into_iter() {
            let url = url.as_ref().to_owned();
            let task: JoinHandle<Result<String>> = tokio::spawn(async move {
                println!("🈷️ - Downloading HSK data from: {url}");
                let res = reqwest::get(url).await?.text().await?;

                Ok(res)
            });

            tasks.push(task);
        }

        for task in tasks {
            let content = task.await??;
            html_content.push(content);
        }

        Ok(Self::set_contents(html_content))
    }
    /// Get the contents of the urls that has been fetched
    ///
    /// # Arguments
    ///
    /// * `self` - self
    async fn get_contents(self) -> Result<HashMap<String, HSKLevel>>;
    /// Set the contents fetch from the urls
    ///
    /// # Arguments
    ///
    /// * `contents` - Vec<String>
    fn set_contents(contents: Vec<String>) -> Self;
}

pub async fn load_hsk_levels() -> Result<HashMap<String, HSKLevel>> {
    let (mut mhsk, whsk) = tokio::try_join!(
        Mandarinbean::download_contents(URLS.to_vec())
            .await?
            .get_contents(),
        Wohok::download_contents(WohokURLS.to_vec())
            .await?
            .get_contents()
    )?;

    mhsk.extend(whsk);

    Ok(mhsk)
}
