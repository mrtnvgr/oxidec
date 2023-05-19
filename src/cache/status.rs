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

    pub fn load() -> Self {
        let path = Self::path();
        let fr = fs::File::open(path).expect("Failed to read the file");
        serde_json::from_reader(fr).expect("Failed to parse the file")
    }

    pub fn save(&self) -> std::io::Result<()> {
        fs::write(Self::path(), self.to_json_string())
    }

    fn path() -> PathBuf {
        let home_dir = home_dir().expect("Failed to get HOME directory");
        let cache_path = ".cache/oxidec/status/colorscheme.json";
        home_dir.join(cache_path)
    }

    fn to_json_string(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}
