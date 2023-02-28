use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetadataConfig {
    pub server_name: String,
    pub folder_name: String,

    pub files_url: String,
    pub files_hash: String,

    pub theme_url: Option<String>,
    pub theme_hash: Option<String>,
}

impl MetadataConfig {
    pub async fn download_from_url(url: &str) -> Result<Self, reqwest::Error> {
        let response = reqwest::get(url).await?;
        let config = response.json::<MetadataConfig>().await?;
        Ok(config)
    }
}
