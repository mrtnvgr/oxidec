use crate::{
    cache::status::{Colorscheme, Wallpaper},
    config::Directory,
};
use serde::{Deserialize, Serialize};
use std::{fs::File, path::Path};

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

    pub fn from_file(path: &Path) -> serde_json::Result<Self> {
        let fr = File::open(path).expect("Failed to read the file");
        serde_json::from_reader(fr)
    }

    pub fn save(self) {
        let path = Directory::States.build_path(&self.name);
        serde_json::to_writer(File::create(path).unwrap(), &self).unwrap();
    }
}
