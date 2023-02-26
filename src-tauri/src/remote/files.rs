use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoteFile {
    pub name: String,
    pub checksum: String,
}
