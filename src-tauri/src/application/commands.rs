use crate::remote::config::Config;

#[derive(Debug, thiserror::Error)]
pub enum CommandError {
    #[error(transparent)]
    DownloadConfig(#[from] reqwest::Error),
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
pub async fn fetch_config(url: String) -> Result<Config, CommandError> {
    let config = Config::download_from_url(&url).await?;

    Ok(config)
}
