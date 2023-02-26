use tauri::Manager;

use crate::remote::meta_config::MetadataConfig;

use super::{config_file::ConfigFile, paths::config_path};

#[derive(Debug, thiserror::Error)]
pub enum CommandError {
    #[error(transparent)]
    DownloadConfig(#[from] reqwest::Error),

    #[error(transparent)]
    ConfigIO(#[from] std::io::Error),

    #[error(transparent)]
    TauriError(#[from] tauri::Error),
}

impl serde::Serialize for CommandError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

#[tauri::command]
pub async fn fetch_config(url: String) -> Result<MetadataConfig, CommandError> {
    let config = MetadataConfig::download_from_url(&url).await?;

    Ok(config)
}

#[tauri::command]
pub async fn create_config(
    app_handle: tauri::AppHandle,
    url: String,
    metadata: MetadataConfig,
) -> Result<(), CommandError> {
    let dir = config_path(&app_handle.path_resolver());

    let mut config = ConfigFile::load(&dir)?;

    config.add_config(&url, &metadata);

    config.save(&dir)?;

    app_handle.emit_all(
        "open-config",
        format!("launcher-bootstrap://open-config?url={}", url),
    )?;

    Ok(())
}
