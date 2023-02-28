use tauri::Manager;
use tracing::trace;

use crate::{
    application::{
        config_file::ConfigFile,
        link_events::{LinkEvent, LinkEventPayload},
        paths::config_path,
        shortcuts::Shortcut,
    },
    remote::meta_config::MetadataConfig,
};

use super::errors::CommandError;

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
