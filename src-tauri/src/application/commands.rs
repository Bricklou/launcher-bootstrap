use tauri::Manager;
use tracing::trace;

use crate::remote::meta_config::MetadataConfig;

use super::{
    config_file::ConfigFile,
    link_events::{LinkEvent, LinkEventPayload},
    paths::config_path,
    shortcuts::{Shortcut, ShortcutError},
};

#[derive(Debug, thiserror::Error)]
pub enum CommandError {
    #[error(transparent)]
    DownloadConfig(#[from] reqwest::Error),

    #[error(transparent)]
    ConfigIO(#[from] std::io::Error),

    #[error(transparent)]
    TauriError(#[from] tauri::Error),

    #[error(transparent)]
    ShortcutError(#[from] ShortcutError),
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
pub fn get_configs(app_handle: tauri::AppHandle) -> Result<ConfigFile, CommandError> {
    let dir = config_path(&app_handle.path_resolver());

    let config = ConfigFile::load(&dir)?;

    Ok(config)
}

#[tauri::command]
pub async fn create_config(
    app_handle: tauri::AppHandle,
    url: String,
    metadata: MetadataConfig,
) -> Result<(), CommandError> {
    let dir = config_path(&app_handle.path_resolver());

    trace!("Config path: {:?}", dir);

    let mut config = ConfigFile::load(&dir)?;

    config.add_config(&url, &metadata);

    config.save(&dir)?;

    Shortcut::create(&url, &metadata)?;

    let window = app_handle.get_window("main").unwrap();

    app_handle.emit_all(
        "link-event",
        LinkEventPayload {
            event_type: LinkEvent::OpenConfig,
            data: url.to_string(),
        },
    )?;
    window.request_user_attention(Some(tauri::UserAttentionType::Informational))?;

    Ok(())
}
