use tauri::Manager;
use tracing::trace;

use crate::{
    application::{
        config_file::ConfigFile,
        link_events::{LinkEvent, LinkEventPayload},
        paths::{config_path, instance_path, updater_path},
        shortcuts::Shortcut,
    },
    remote::{meta_config::MetadataConfig, updater_config::UpdaterConfig},
};

use super::errors::CommandError;

#[tauri::command]
pub async fn create_config(
    app_handle: tauri::AppHandle,
    url: String,
    metadata: MetadataConfig,
) -> Result<(), CommandError> {
    let config_path = config_path(&app_handle.path_resolver());

    trace!("Config path: {:?}", config_path);

    let mut config = ConfigFile::load(&config_path)?;
    config.add_config(&url, &metadata);

    let instance_path = instance_path(&app_handle.path_resolver(), &metadata);

    if metadata.theme_url.is_some() {
        let updater_path = updater_path(&app_handle.path_resolver(), &metadata);
        let updater = UpdaterConfig::download_from_meta(&url, &metadata).await?;
        updater.save(&updater_path)?;
    }

    config.save(&config_path)?;

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
