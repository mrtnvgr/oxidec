use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
pub struct Colorscheme {
    pub special: Special,
    pub colors: Colors,
}

#[derive(Serialize, Deserialize)]
pub struct Special {
    pub background: String,
    pub foreground: String,
    pub cursor: String,
}

#[derive(Serialize, Deserialize)]
pub struct Colors {
    pub color0: String,
    pub color1: String,
    pub color2: String,
    pub color3: String,
    pub color4: String,
    pub color5: String,
    pub color6: String,
    pub color7: String,
    pub color8: String,
    pub color9: String,
    pub color10: String,
    pub color11: String,
    pub color12: String,
    pub color13: String,
    pub color14: String,
    pub color15: String,
}

impl Colorscheme {
    pub fn from_file(path: &PathBuf) -> serde_json::Result<Self> {
        let fr = std::fs::File::open(path).expect("Failed to read the file");
        serde_json::from_reader(fr)
    }
}
