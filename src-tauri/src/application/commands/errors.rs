use crate::application::shortcuts::ShortcutError;

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
