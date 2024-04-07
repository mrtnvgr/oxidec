use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs::File, path::PathBuf};

#[derive(Serialize, Deserialize)]
pub struct Colorscheme {
    pub special: Special,
    pub palette: HashMap<String, String>,
}

#[derive(Serialize, Deserialize)]
pub struct Special {
    pub background: String,
    pub foreground: String,
    pub cursor: String,
}

impl Colorscheme {
    pub fn from_file(path: &PathBuf) -> serde_json::Result<Self> {
        let fr = File::open(path).expect("Failed to read the file");
        serde_json::from_reader(fr)
    }

    pub fn from_vec_16(colors: Vec<String>) -> Self {
        assert!(colors.len() == 16, "Couldn't generate a colorscheme");

        let mut palette = HashMap::new();
        for (index, value) in colors.into_iter().enumerate() {
            let key = format!("color{index}");
            palette.insert(key, value);
        }

        // PERF: use references
        #[allow(clippy::indexing_slicing)]
        let special = Special {
            background: palette["color0"].clone(),
            foreground: palette["color15"].clone(),
            cursor: palette["color15"].clone(),
        };

        Self { special, palette }
    }
}
