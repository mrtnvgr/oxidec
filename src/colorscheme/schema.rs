use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs::File, path::Path};

#[derive(Serialize, Deserialize)]
pub struct Colorscheme {
    data: HashMap<String, String>,
}

impl Colorscheme {
    pub fn from_file(path: &Path) -> serde_json::Result<Self> {
        let fr = File::open(path).expect("Failed to read the file");
        serde_json::from_reader(fr)
    }

    pub fn from_vec_16(colors: Vec<String>) -> Self {
        assert!(colors.len() == 16, "Couldn't generate a colorscheme");

        let mut data = HashMap::new();
        for (index, value) in colors.into_iter().enumerate() {
            let key = format!("color{index}");
            data.insert(key, value);
        }

        Self { data }
    }
}
