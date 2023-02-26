use std::path::PathBuf;
use tauri::PathResolver;

#[inline]
pub fn config_path(resolver: &PathResolver) -> PathBuf {
    resolver.app_data_dir().unwrap().join("config.json")
}
