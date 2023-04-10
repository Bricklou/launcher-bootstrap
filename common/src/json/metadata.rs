use serde::{Deserialize, Serialize};

use crate::traits::JsonFile;

#[derive(Serialize, Deserialize, Debug)]
pub struct MetadataJson {
    pub server_name: String,
    pub folder_name: String,
}

impl Default for MetadataJson {
    fn default() -> Self {
        Self {
            server_name: "My Server".to_string(),
            folder_name: "my-server".to_string(),
        }
    }
}

impl JsonFile for MetadataJson {
    fn from_json_file(path: std::path::PathBuf) -> Result<Self, Box<dyn std::error::Error>>
    where
        Self: Sized,
    {
        let file = std::fs::File::open(path)?;
        let reader = std::io::BufReader::new(file);
        let metadata = serde_json::from_reader(reader)?;
        Ok(metadata)
    }

    fn to_json_file(self, path: std::path::PathBuf) -> Result<String, Box<dyn std::error::Error>> {
        let json = serde_json::to_string_pretty(&self)?;
        std::fs::write(path, json.clone())?;
        Ok(json)
    }
}
