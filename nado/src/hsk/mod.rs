use anyhow::Result;
use async_trait::async_trait;
use mandarinbean::{Mandarinbean, URLS};
use serde::Serialize;
use std::collections::HashMap;
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

impl ToString for HSKLevel {
    fn to_string(&self) -> String {
        match self {
            Self::HSK1 => "hsk1".to_owned(),
            Self::HSK2 => "hsk2".to_owned(),
            Self::HSK3 => "hsk3".to_owned(),
            Self::HSK4 => "hsk4".to_owned(),
            Self::HSK5 => "hsk5".to_owned(),
            Self::HSK6 => "hsk6".to_owned(),
            Self::HSK7 => "hsk7".to_owned()
        }
    }
}

#[async_trait]
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
