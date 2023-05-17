use crate::config::Folder;
use home::home_dir;
use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::{Path, PathBuf},
};

#[derive(Serialize, Deserialize)]
pub struct Colorscheme {
    pub name: String,
    pub path: PathBuf,
}

impl Colorscheme {
    pub fn new(entry: &str) -> Self {
        let stem = Path::new(&entry).file_stem().unwrap();
        let name = stem.to_string_lossy().to_string();
        let path = Folder::Colorschemes.get(entry).unwrap();
        Self { name, path }
    }

    pub fn save(&self) -> std::io::Result<()> {
        let path = home_dir()
            .expect("Failed to get HOME directory")
            .join(".cache/oxidec/status/colorscheme.json");
        fs::write(path, self.to_json_string())
    }

    fn to_json_string(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}
