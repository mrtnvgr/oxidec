use crate::cache::status::{Colorscheme, Wallpaper};
use serde::{Deserialize, Serialize};
use std::{fs::File, path::Path};

#[derive(Serialize, Deserialize)]
pub struct Theme {
    pub name: String,
    pub colorscheme: Colorscheme,
    pub wallpapers: Vec<Wallpaper>,
}

impl Theme {
    pub fn new(name: &str, colorscheme: Colorscheme, wallpapers: Vec<Wallpaper>) -> Self {
        Self {
            name: name.to_owned(),
            colorscheme,
            wallpapers,
        }
    }

    pub fn from_file(path: &Path) -> serde_json::Result<Self> {
        let fr = File::open(path).expect("Failed to read the file");
        serde_json::from_reader(fr)
    }
}
