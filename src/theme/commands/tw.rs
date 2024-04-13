use crate::{
    cache::status::{self, Object},
    theme::{args::ThemeWallpapers, schema},
    wallpaper,
};
use rand::prelude::*;

pub fn handle(kind: &ThemeWallpapers) {
    match kind {
        ThemeWallpapers::Add | ThemeWallpapers::Remove => toggle(kind),
        ThemeWallpapers::Rnd => set_random(),
    }
}

fn toggle(kind: &ThemeWallpapers) {
    let cache = status::Theme::load();
    let wallpaper = status::Wallpaper::load();

    let error_message = format!("Failed to load {:?}", cache.name);
    let mut theme = schema::Theme::from_file(&cache.path).expect(&error_message);

    let contains = theme.wallpapers.iter().any(|x| *x.path == wallpaper.path);

    match kind {
        ThemeWallpapers::Add if !contains => theme.wallpapers.push(wallpaper),
        ThemeWallpapers::Add => panic!("Current theme already has this wallpaper"),
        ThemeWallpapers::Remove => theme.wallpapers.retain(|x| *x.path != wallpaper.path),
        ThemeWallpapers::Rnd => unreachable!(),
    }

    theme.save();

    log::info!("Success");
}

fn set_random() {
    let cache = status::Theme::load();

    let error_message = format!("Failed to load {:?}", cache.name);
    let theme = schema::Theme::from_file(&cache.path).expect(&error_message);

    let mut rng = thread_rng();
    let wallpaper = theme.wallpapers.choose(&mut rng).unwrap();

    wallpaper::set(&wallpaper.name, wallpaper.mode);
}
