#[derive(Debug, thiserror::Error)]
pub enum DownloadError {
    #[error("Failed to download file: {0}")]
    DownloadError(#[from] reqwest::Error),
    #[error("Failed to save file: {0}")]
    SaveError(#[from] std::io::Error),
    #[error("Failed to parse url: {0}")]
    UrlParseError(#[from] url::ParseError),
}

pub async fn download_and_check_hash(
    url: &str,
    hash: &str,
    path: &std::path::Path,
) -> Result<(), DownloadError> {
    unimplemented!("Download and check hash");
}
