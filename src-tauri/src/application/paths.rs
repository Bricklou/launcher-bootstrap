use std::path::PathBuf;
use tauri::PathResolver;

use crate::remote::meta_config::MetadataConfig;

#[inline]
pub fn config_path(resolver: &PathResolver) -> PathBuf {
    resolver.app_data_dir().unwrap().join("config.json")
}

pub fn instance_path(resolver: &PathResolver, metadata: &MetadataConfig) -> PathBuf {
    resolver.app_data_dir().unwrap().join(&metadata.folder_name)
}

#[inline]
pub fn updater_path(resolver: &PathResolver, metadata: &MetadataConfig) -> PathBuf {
    instance_path(resolver, metadata).join("updater.json")
}

#[inline]
pub fn files_path(resolver: &PathResolver, metadata: &MetadataConfig) -> PathBuf {
    instance_path(resolver, metadata).join("files.json")
}
