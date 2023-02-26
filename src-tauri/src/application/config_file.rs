use std::{collections::HashMap, io::Write, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::remote::meta_config::MetadataConfig;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigFile {
    configs: HashMap<String, MetadataConfig>,
}

impl ConfigFile {
    pub fn new() -> Self {
        Self {
            configs: HashMap::new(),
        }
    }

    pub fn add_config<S>(&mut self, url: S, config: &MetadataConfig)
    where
        S: Into<String>,
    {
        self.configs.insert(url.into(), config.clone());
    }

    pub fn remove_config(&mut self, name: &str) {
        self.configs.remove(name);
    }

    pub fn get_config(&self, url: &str) -> Option<&MetadataConfig> {
        self.configs.get(url)
    }

    pub fn get_configs(&self) -> &HashMap<String, MetadataConfig> {
        &self.configs
    }

    pub fn load(config_path: &PathBuf) -> Result<Self, std::io::Error> {
        if !config_path.exists() {
            return Ok(Self::new());
        }

        let contents = std::fs::read_to_string(config_path)?;
        let config_file: ConfigFile = serde_json::from_str(&contents)?;

        Ok(config_file)
    }

    pub fn save(&self, config_path: &PathBuf) -> Result<(), std::io::Error> {
        let mut file = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(config_path)?;

        let contents = serde_json::to_string_pretty(&self)?;

        file.write_all(contents.as_bytes())?;

        Ok(())
    }
}
