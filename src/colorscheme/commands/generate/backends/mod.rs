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

        if let Some(saturate) = args.saturate {
            saturate_colors(&mut colors, saturate);
        }

        let hex_colors: Vec<String> = colors.iter().map(Rgb::to_hex_string).collect();
        schema::Colorscheme::from_vec_16(&hex_colors)
    }
}

fn saturate_colors(colors: &mut [Rgb], saturate: i16) {
    for (i, color) in colors.iter_mut().enumerate() {
        if ![0, 7, 8, 15].contains(&i) {
            let saturation = SaturationInSpace::Hsl(saturate.into());
            color.saturate(saturation);
        }
    }
}
