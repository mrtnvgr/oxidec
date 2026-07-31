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
        .unwrap_or_else(|| {
            if let Some(current) = cache::status::Theme::try_load() {
                Directory::Themes.random_entry_excluding(&current.name)
            } else {
                Directory::Themes.random_entry()
            }
        });

    assert!(
        Directory::Themes.contains(&name),
        "This theme does not exist"
    );

    let path = Directory::Themes.get(&name).unwrap();

    let error_message = format!("Failed to load {name:?}");
    let theme = schema::Theme::from_file(&path).expect(&error_message);

    let cache = cache::status::Theme::new(&name);
    cache.save().unwrap();

    colorscheme::set_without_cache(&theme.colorscheme);

    let mut rng = rand::rng();
    let wallpaper = theme.wallpapers.choose(&mut rng).unwrap();

    let wallpaper_name = wallpaper.path.to_string_lossy();

    wallpaper::set(wallpaper.path.clone(), wallpaper.mode);

    log::info!("Current theme: {}", name);
    log::info!("Current wallpaper: {}", wallpaper_name);

    colorscheme::blocks::print();
}
