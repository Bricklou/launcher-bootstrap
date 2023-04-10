use std::{
    path::{Path, PathBuf},
    vec,
};

use common::{json::metadata::MetadataJson, traits::JsonFile};
use requestty::Question;

use crate::autocomplete::auto_complete;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let metadata = ask_metadata_infos()?;

    println!("Metadata: {:?}", metadata);

    let save_to = requestty::Question::input("save_to")
        .message("Save into ?")
        .default("./output")
        .auto_complete(|p, _| auto_complete(p))
        .validate(|p, _| {
            let pa = p.as_ref() as &Path;
            if pa.exists() {
                if pa.is_file() {
                    return Err(format!("`{}` is a file", p));
                }
            }
            Ok(())
        })
        .build();

    let answers = requestty::prompt_one(save_to)?;
    let path = answers.as_string().unwrap();
    let path = PathBuf::from(path);

    // Create output folder if it doesn't exist
    if !path.exists() {
        std::fs::create_dir_all(&path)?;
    }

    let meta_path = path.join("metadata.json");

    metadata.to_json_file(meta_path)?;
    Ok(())
}

fn ask_metadata_infos() -> Result<MetadataJson, Box<dyn std::error::Error>> {
    let meta_path = PathBuf::from("./metadata.json");
    let metadata = if meta_path.exists() {
        MetadataJson::from_json_file(meta_path)?
    } else {
        MetadataJson::default()
    };

    let module = requestty::PromptModule::new(vec![
        Question::input("server_name")
            .message("Server name ?")
            .default(metadata.server_name)
            .build(),
        Question::input("folder_name")
            .message("Folder name ?")
            .default(metadata.folder_name)
            .build(),
    ]);

    let answers = module.prompt_all()?;

    let server_name = answers.get("server_name").unwrap().as_string();
    let folder_name = answers.get("folder_name").unwrap().as_string();

    let metadata = MetadataJson {
        server_name: server_name.unwrap().to_string(),
        folder_name: folder_name.unwrap().to_string(),
    };

    Ok(metadata)
}
