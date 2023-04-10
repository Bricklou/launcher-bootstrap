use std::path::PathBuf;

pub trait JsonFile {
    fn from_json_file(path: PathBuf) -> Result<Self, Box<dyn std::error::Error>>
    where
        Self: Sized;
    fn to_json_file(self, path: PathBuf) -> Result<String, Box<dyn std::error::Error>>;
}
