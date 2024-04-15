use serde::{Deserialize, Serialize};
use std::{fs::File, path::Path};

use crate::{
    cache::status::{Colorscheme, Wallpaper},
    config::Directory,
};

#[derive(Serialize, Deserialize)]
pub struct Theme {
    pub colorscheme: Colorscheme,
    pub wallpapers: Vec<Wallpaper>,
}

impl Theme {
    pub fn new(colorscheme: Colorscheme, wallpapers: Vec<Wallpaper>) -> Self {
        Self {
            colorscheme,
            wallpapers,
        }
    }

    pub fn from_file(path: &Path) -> serde_json::Result<Self> {
        let fr = File::open(path).expect("Failed to read the file");
        serde_json::from_reader(fr)
    }

    pub fn save(self, name: &str) {
        let path = Directory::Themes.build_path(name);
        serde_json::to_writer(File::create(path).unwrap(), &self).unwrap();
    }
}
