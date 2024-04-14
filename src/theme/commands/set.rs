use crate::{
    cache::{self, status::Object},
    colorscheme,
    config::Directory,
    theme::{args, schema},
    wallpaper,
};

use rand::prelude::*;

pub fn handle(args: &args::Set) {
    let name = args
        .name
        .clone()
        .unwrap_or_else(|| Directory::Themes.random_entry());

    assert!(
        Directory::Themes.contains(&name),
        "This theme does not exist"
    );

    let path = Directory::Themes.get(&name).unwrap();

    let error_message = format!("Failed to load {name:?}");
    let theme = schema::Theme::from_file(&path).expect(&error_message);

    colorscheme::set(&theme.colorscheme.name, false);

    let mut rng = rand::thread_rng();
    let wallpaper = theme.wallpapers.choose(&mut rng).unwrap();

    let wallpaper_name = wallpaper.path.to_string_lossy();

    wallpaper::set(wallpaper.path.clone(), wallpaper.mode);

    let cache = cache::status::Theme::new(&name);
    cache.save().unwrap();

    log::info!("Current colorscheme: {}", theme.colorscheme.name);
    log::info!("Current wallpaper: {}", wallpaper_name);
    eprintln!();
    log::info!("Current theme: {}", name);

    colorscheme::blocks::print();
}
