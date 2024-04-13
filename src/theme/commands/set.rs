use crate::{
    colorscheme,
    config::Folder,
    theme::{args, schema},
    wallpaper,
};

use rand::prelude::*;

pub fn handle(args: &args::Set) {
    let name = args.name.clone().unwrap_or_else(get_random_theme);

    assert!(Folder::Themes.contains(&name), "This theme does not exist");

    let path = Folder::Themes.get(&name).unwrap();

    let error_message = format!("Failed to load {name:?}");
    let theme = schema::Theme::from_file(&path).expect(&error_message);

    colorscheme::set(&theme.colorscheme.name, false);

    let mut rng = rand::thread_rng();
    let wallpaper = theme.wallpapers.choose(&mut rng).unwrap();

    wallpaper::set(&wallpaper.name, wallpaper.mode);

    log::info!("Current colorscheme: {}", theme.colorscheme.name);
    log::info!("Current wallpaper: {}", wallpaper.name);

    colorscheme::blocks::print();
}

fn get_random_theme() -> String {
    let file = Folder::Themes.random_file().expect("There are no themes.");
    let filestem = file.file_stem().unwrap();
    filestem.to_str().unwrap().to_owned()
}
