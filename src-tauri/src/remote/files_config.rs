use serde::{Deserialize, Serialize};
use time::{self, OffsetDateTime};

use super::files::RemoteFile;

// Configuration object which will represent the files to update and their checksums.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LauncherFilesConfig {
    pub files: Vec<RemoteFile>,
    #[serde(with = "time::serde::rfc3339")]
    pub last_modified: OffsetDateTime,
}
