pub mod backends;

use crate::{
    cache::status::{self, Object},
    colorscheme::args,
    config::Directory,
};
use std::fs::File;

pub fn handle(args: &args::Generate) {
    log::info!("Generating colorscheme...");
    let wallpaper = status::Wallpaper::load();

    let name = wallpaper.path.to_string_lossy();

    assert!(
        !Directory::Colorschemes.contains(&name),
        "Colorscheme with this name already exists!"
    );

    let colorscheme = args.backend.generate(&wallpaper.path, args);

    let path = Directory::Colorschemes.build_path(&name);
    serde_json::to_writer(File::create(path).unwrap(), &colorscheme).unwrap();

    log::info!("Generated colorscheme: {}", name);
}
