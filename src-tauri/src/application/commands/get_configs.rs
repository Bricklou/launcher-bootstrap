use crate::application::{config_file::ConfigFile, paths::config_path};

use super::errors::CommandError;

#[tauri::command]
pub fn get_configs(app_handle: tauri::AppHandle) -> Result<ConfigFile, CommandError> {
    let dir = config_path(&app_handle.path_resolver());

    let config = ConfigFile::load(&dir)?;

    Ok(config)
}
