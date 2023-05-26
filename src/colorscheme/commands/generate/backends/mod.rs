use crate::colorscheme::{args, schema};
use clap::ValueEnum;
use colorsys::{ColorTransform, Rgb, SaturationInSpace};
use std::path::Path;

mod imagemagick;

#[derive(ValueEnum, Clone, Copy)]
pub enum Backend {
    Imagemagick,
}

impl Backend {
    pub fn generate(self, path: &Path, args: &args::Generate) -> schema::Colorscheme {
        let mut colors = match self {
            Self::Imagemagick => imagemagick::generate(path, args.light),
        };

        let hex_colors: Vec<String> = colors.iter().map(Rgb::to_hex_string).collect();
        schema::Colorscheme::from_vec_16(&hex_colors)
    }
}
