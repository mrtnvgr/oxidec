use std::fs::File;

use crate::{
    cache::status,
    config::Folder,
    theme::{args, schema},
};

pub fn handle(args: &args::Save) {
    assert!(
        !Folder::Themes.contains(&args.name),
        "Theme with this name already exists!"
    );

    let colorscheme = status::Colorscheme::load();
    let wallpaper = status::Wallpaper::load();

    let theme = schema::Theme::new(&args.name, colorscheme, wallpaper);

    let path = Folder::Themes.build_path(&args.name);
    serde_json::to_writer(File::create(path).unwrap(), &theme).unwrap();

    log::info!("Saved successfully!");
}
