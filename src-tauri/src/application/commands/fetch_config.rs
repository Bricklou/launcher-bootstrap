use crate::remote::meta_config::MetadataConfig;

use super::errors::CommandError;

#[tauri::command]
pub async fn fetch_config(url: String) -> Result<MetadataConfig, CommandError> {
    let config = MetadataConfig::download_from_url(&url).await?;

    Ok(config)
}
