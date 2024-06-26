use crate::config::Directory;
use clap::ValueEnum;
use home::home_dir;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::{
    fs, io,
    path::{Path, PathBuf},
};

pub trait Object
where
    Self: DeserializeOwned + Serialize,
{
    const NAME: &'static str;

    fn load() -> Self {
        let path = Self::path();

        let reader = match fs::File::open(path) {
            Ok(fr) => fr,
            Err(err) if err.kind() == io::ErrorKind::NotFound => {
                panic!("{} status file does not exist", Self::NAME)
            }
            _ => panic!("Failed to read the {} status file", Self::NAME),
        };

        serde_json::from_reader(reader).expect("Failed to parse status")
    }

    fn path() -> PathBuf {
        let home_dir = home_dir().expect("Failed to get HOME directory");
        let cache_path = format!(".cache/oxidec/status/{}.json", Self::NAME);
        home_dir.join(cache_path)
    }

    fn save(&self) -> io::Result<()> {
        fs::write(Self::path(), self.to_json_string())
    }

    fn to_json_string(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

#[derive(Serialize, Deserialize)]
pub struct Colorscheme {
    pub name: String,
    pub path: PathBuf,
}

#[derive(Serialize, Deserialize, PartialEq)]
pub struct Wallpaper {
    pub path: PathBuf,
    pub mode: WallpaperMode,
}

#[derive(Serialize, Deserialize)]
pub struct Theme {
    pub name: String,
    pub path: PathBuf,
}

impl Object for Colorscheme {
    const NAME: &'static str = "colorscheme";
}

impl Object for Wallpaper {
    const NAME: &'static str = "wallpaper";
}

impl Object for Theme {
    const NAME: &'static str = "theme";
}

impl Colorscheme {
    pub fn new(entry: &str) -> Self {
        let stem = Path::new(&entry).file_stem().unwrap();
        let name = stem.to_string_lossy().to_string();
        let path = Directory::Colorschemes.get(entry).unwrap();
        Self { name, path }
    }
}

impl Wallpaper {
    pub fn new(path: PathBuf, mode: WallpaperMode) -> Self {
        Self { path, mode }
    }
}

impl Theme {
    pub fn new(entry: &str) -> Self {
        let stem = Path::new(&entry).file_stem().unwrap();
        let name = stem.to_string_lossy().to_string();
        let path = Directory::Themes.get(entry).unwrap();
        Self { name, path }
    }
}

#[derive(Serialize, Deserialize, ValueEnum, Clone, Copy, Debug, PartialEq)]
pub enum WallpaperMode {
    // TODO: visible alias
    #[value(alias = "centre")]
    Center,
    Fill,
    Max,
    Scale,
    Tile,
}
