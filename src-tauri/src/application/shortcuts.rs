use crate::remote::meta_config::MetadataConfig;

#[derive(Debug, thiserror::Error)]
pub enum ShortcutError {
    #[error("Couldn't create shortcut: {0}")]
    CreateShortcut(#[from] std::io::Error),

    #[cfg(target_os = "windows")]
    #[error("Couldn't create shortcut: {0}")]
    CreateShortcutWindows(#[from] mslnk::MSLinkError),
}

pub struct Shortcut;

impl Shortcut {
    #[cfg(target_os = "linux")]
    pub fn create(url: &str, config: &MetadataConfig) -> Result<(), ShortcutError> {
        use std::fs::File;
        use std::io::{Error, ErrorKind, Write};
        use tauri::api::path::data_dir;

        let exe = tauri::utils::platform::current_exe()?;

        let mut target = data_dir()
            .ok_or_else(|| Error::new(ErrorKind::NotFound, "data directory not found."))?
            .join("applications");

        std::fs::create_dir_all(&target)?;

        let file_name = format!(
            "{}-{}.desktop",
            exe.file_name()
                .ok_or_else(|| Error::new(
                    ErrorKind::NotFound,
                    "Couldn't get file name of curent executable.",
                ))?
                .to_string_lossy(),
            config.folder_name
        );

        target.push(format!("{}.desktop", file_name));

        let mut file = File::create(&target)?;
        file.write_all(
            format!(
                include_str!("shortcut_template.desktop"),
                name = config.server_name,
                exec = exe.to_string_lossy(),
                url = format!("launcher-bootstrap://open-config?url={}", url),
            )
            .as_bytes(),
        )?;

        Ok(())
    }

    #[cfg(target_os = "macos")]
    pub fn create(
        url: &str,
        config: &MetadataConfig,
        resolver: &PathResolver,
    ) -> Result<(), ShortcutError> {
        let local_apps_path = resolver.app_local_data_dir();
        Ok(())
    }

    #[cfg(target_os = "windows")]
    pub fn create(url: &str, config: &MetadataConfig) -> Result<(), ShortcutError> {
        use std::io;

        use mslnk::ShellLink;
        use tauri::api::path::desktop_dir;

        let exe = tauri::utils::platform::current_exe()?.canonicalize()?;

        let target = format!("launcher-bootstrap://open-config?url={}", url);

        let link = desktop_dir()
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Desktop directory not found."))?
            .join(format!("{}.lnk", config.server_name));

        if link.exists() {
            std::fs::remove_file(&link)?;
        }

        let mut shortcut = ShellLink::new(exe.to_string_lossy().replace("\\\\?\\", ""))?;
        shortcut.set_arguments(Some(target));
        shortcut.create_lnk(link)?;
        Ok(())
    }
}
