use crate::colorscheme::schema;
use colorsys::Rgb;
use serde::{Deserialize, Serialize};
use std::{path::Path, str::FromStr};

mod imagemagick;

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub enum Backend {
    ImageMagick,
}

impl FromStr for Backend {
    type Err = &'static str;
    fn from_str(mode: &str) -> Result<Self, Self::Err> {
        match mode {
            "imagemagick" => Ok(Self::ImageMagick),
            _ => Err("This backend doesn't exist"),
        }
    }
}

impl Backend {
    pub const fn variants() -> [&'static str; 1] {
        ["imagemagick"]
    }

    pub fn generate(self, path: &Path, light: bool) -> schema::Colorscheme {
        let colors = match self {
            Self::ImageMagick => imagemagick::generate(path, light),
        };

        let hex_colors: Vec<String> = colors.iter().map(Rgb::to_hex_string).collect();
        schema::Colorscheme::from_vec_16(&hex_colors)
    }
}
