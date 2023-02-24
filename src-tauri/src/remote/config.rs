use serde::{Deserialize, Serialize};
use time::{self, OffsetDateTime};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub server_name: String,
    pub folder_name: String,
    pub files: Vec<String>,
    #[serde(with = "time::serde::rfc3339")]
    pub last_modified: OffsetDateTime,
}

impl Config {
    pub async fn download_from_url(url: &str) -> Result<Self, reqwest::Error> {
        let response = reqwest::get(url).await?;
        let config = response.json::<Config>().await?;
        Ok(config)
    }
}
