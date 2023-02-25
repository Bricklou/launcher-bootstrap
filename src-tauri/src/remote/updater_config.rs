use serde::{Deserialize, Serialize};
use time::{self, OffsetDateTime};

use super::files::RemoteFile;

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdaterConfig {
    #[serde(with = "time::serde::rfc3339")]
    pub last_modified: OffsetDateTime,
    pub files: Vec<RemoteFile>,
    pub index: String,
}
