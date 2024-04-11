use crate::cache::status::{Colorscheme, Wallpaper};
use serde::{Deserialize, Serialize};
use std::{fs::File, path::PathBuf};

#[derive(Serialize, Deserialize)]
pub struct State {
    pub name: String,
    pub colorscheme: Colorscheme,
    pub wallpaper: Wallpaper,
}

impl State {
    pub fn new(name: &str, colorscheme: Colorscheme, wallpaper: Wallpaper) -> Self {
        Self {
            name: name.to_owned(),
            colorscheme,
            wallpaper,
        }
    }

    pub fn from_file(path: &PathBuf) -> serde_json::Result<Self> {
        let fr = File::open(path).expect("Failed to read the file");
        serde_json::from_reader(fr)
    }
}
