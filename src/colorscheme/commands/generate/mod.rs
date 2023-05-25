pub mod backends;

use crate::{cache::status, colorscheme::args, config::Folder};
use std::{fs::File, path::Path};

pub fn handle(args: &args::Generate) {
    log::info!("Generating colorscheme...");
    let wallpaper = status::Wallpaper::load();

    let file = Path::new(&wallpaper.name);
    let filestem = file.file_stem().unwrap();
    let name = filestem.to_str().unwrap();

    assert!(
        !Folder::Colorschemes.contains(name),
        "Colorscheme with this name already exists!"
    );

    let colorscheme = args.backend.generate(&wallpaper.path, args.light);

    let path = Folder::Colorschemes.build_path(name);
    serde_json::to_writer(File::create(path).unwrap(), &colorscheme).unwrap();

    log::info!("Generated theme: {}", name);
}
