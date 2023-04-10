use std::path::Path;

use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};
use requestty::question::{completions, Completions};

pub fn auto_complete(p: String) -> Completions<String> {
    let current: &Path = p.as_ref();
    let (mut dir, last) = if p.ends_with('/') {
        (current, "")
    } else {
        let dir = current.parent().unwrap_or_else(|| "/".as_ref());
        let last = current
            .file_name()
            .and_then(std::ffi::OsStr::to_str)
            .unwrap_or("");
        (dir, last)
    };

    if dir.to_str().unwrap().is_empty() {
        dir = ".".as_ref();
    }

    let mut files: Completions<_> = match dir.read_dir() {
        Ok(files) => files
            .flatten()
            .flat_map(|file| {
                let path = file.path();
                let is_dir = path.is_dir();
                match path.into_os_string().into_string() {
                    Ok(s) if is_dir => Some(s + "/"),
                    Ok(s) => Some(s),
                    Err(_) => None,
                }
            })
            .collect(),
        Err(_) => {
            return completions![p];
        }
    };

    if files.is_empty() {
        return completions![p];
    }

    let fuzzer = SkimMatcherV2::default();
    files.sort_by_cached_key(|file| fuzzer.fuzzy_match(file, last).unwrap_or(i64::MAX));
    files
}
