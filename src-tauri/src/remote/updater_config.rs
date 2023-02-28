use std::io::Write;

use serde::{Deserialize, Serialize};
use time::{self, OffsetDateTime};

use super::{files::RemoteFile, meta_config::MetadataConfig};

#[derive(Debug, thiserror::Error)]
pub enum UpdaterConfigError {
    #[error("Failed to download updater config: {0}")]
    DownloadError(#[from] reqwest::Error),
    #[error("Failed to save updater config: {0}")]
    SaveError(#[from] std::io::Error),
    #[error("Failed to parse url: {0}")]
    UrlParseError(#[from] url::ParseError),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdaterConfig {
    #[serde(with = "time::serde::rfc3339")]
    pub last_modified: OffsetDateTime,
    pub files: Vec<RemoteFile>,
    pub index: String,
}

impl UpdaterConfig {
    pub async fn download_from_meta(
        base_url: &String,
        meta: &MetadataConfig,
    ) -> Result<Self, UpdaterConfigError> {
        dbg!(&base_url);
        if meta.theme_url.is_none() {
            return Err(UpdaterConfigError::UrlParseError(
                url::ParseError::EmptyHost,
            ));
        }

        let theme_url = meta.theme_url.clone().unwrap();

        let url = url::Url::parse(&base_url)?.join(&theme_url)?;

        let response = reqwest::get(url).await?;
        let config = response.json::<UpdaterConfig>().await?;
        Ok(config)
    }

    pub fn save(&self, updater_path: &std::path::Path) -> Result<(), std::io::Error> {
        let parent = updater_path.parent().unwrap();
        if !parent.exists() {
            std::fs::create_dir_all(parent)?;
        }

        let mut file = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(updater_path)?;

        let contents = serde_json::to_string_pretty(&self)?;
        file.write_all(contents.as_bytes())?;
        Ok(())
    }
}
