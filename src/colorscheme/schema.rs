use serde::{Deserialize, Serialize};
use std::{fs::File, path::PathBuf};

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
        let fr = File::open(path).expect("Failed to read the file");
        serde_json::from_reader(fr)
    }

    pub fn from_vec_16(colors: &[String]) -> Self {
        assert!(colors.len() == 16, "Couldn't generate a colorscheme");

        #[allow(clippy::indexing_slicing)]
        let colors = Colors {
            color0: colors[0].clone(),
            color1: colors[1].clone(),
            color2: colors[2].clone(),
            color3: colors[3].clone(),
            color4: colors[4].clone(),
            color5: colors[5].clone(),
            color6: colors[6].clone(),
            color7: colors[7].clone(),
            color8: colors[8].clone(),
            color9: colors[9].clone(),
            color10: colors[10].clone(),
            color11: colors[11].clone(),
            color12: colors[12].clone(),
            color13: colors[13].clone(),
            color14: colors[14].clone(),
            color15: colors[15].clone(),
        };

        let special = Special {
            background: colors.color0.clone(),
            foreground: colors.color15.clone(),
            cursor: colors.color15.clone(),
        };

        Self { special, colors }
    }
}
