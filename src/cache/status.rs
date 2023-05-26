use crate::config::Folder;
use clap::ValueEnum;
use home::home_dir;
use serde::{Deserialize, Serialize};
use std::{
    fs,
    io::ErrorKind,
    path::{Path, PathBuf},
};

// FIXME: Rework this!

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

        let fr = match fs::File::open(path) {
            Ok(fr) => fr,
            Err(err) if err.kind() == ErrorKind::NotFound => {
                panic!("Colorscheme status file does not exist")
            }
            _ => panic!("Failed to read the colorscheme status file"),
        };

        serde_json::from_reader(fr).expect("Failed to parse the colorscheme status")
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

#[derive(Serialize, Deserialize)]
pub struct Wallpaper {
    pub name: String,
    pub path: PathBuf,
    pub mode: WallpaperMode,
}

impl Wallpaper {
    pub fn new(entry: &str, mode: WallpaperMode) -> Self {
        let name = Path::new(&entry).file_name().unwrap();
        let name = name.to_string_lossy().to_string();
        let path = Folder::Wallpapers.get(entry).unwrap();
        Self { name, path, mode }
    }

    pub fn load() -> Self {
        let path = Self::path();
        let fr = match fs::File::open(path) {
            Ok(fr) => fr,
            Err(err) if err.kind() == ErrorKind::NotFound => {
                panic!("Wallpaper status file does not exist")
            }
            _ => panic!("Failed to read the wallpaper status file"),
        };

        serde_json::from_reader(fr).expect("Failed to parse the wallpaper status")
    }

    pub fn save(&self) -> std::io::Result<()> {
        fs::write(Self::path(), self.to_json_string())
    }

    fn path() -> PathBuf {
        let home_dir = home_dir().expect("Failed to get HOME directory");
        let cache_path = ".cache/oxidec/status/wallpaper.json";
        home_dir.join(cache_path)
    }

    fn to_json_string(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

#[derive(Serialize, Deserialize, ValueEnum, Clone, Copy, Debug)]
pub enum WallpaperMode {
    // TODO: visible alias
    #[value(alias = "centre")]
    Center,
    Fill,
    Max,
    Scale,
    Tile,
}
