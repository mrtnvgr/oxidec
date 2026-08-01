use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs::File, path::Path};

#[derive(Serialize, Deserialize)]
pub struct Colorscheme {
    #[serde(flatten)]
    data: HashMap<String, String>,
}

impl Colorscheme {
    pub fn from_file(name: String, path: &Path) -> Self {
        let error_message = format!("Failed to load {name:?}");

        let fr = File::open(path).expect(&error_message);
        let mut colorscheme: Self = serde_json::from_reader(fr).expect(&error_message);

        macro_rules! s { ($x:expr) => { $x.to_owned() }; }
        colorscheme.data.entry(s!("name")).or_insert(name);

        colorscheme
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
