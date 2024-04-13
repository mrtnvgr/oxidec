use crate::{
    cache::status::{self, Object},
    theme::{args::ThemeWallpapers, schema},
};

pub fn handle(kind: &ThemeWallpapers) {
    match kind {
        ThemeWallpapers::TWAdd | ThemeWallpapers::TWRemove => toggle(kind),
    }
}

fn toggle(kind: &ThemeWallpapers) {
    let cache = status::Theme::load();
    let wallpaper = status::Wallpaper::load();

    let error_message = format!("Failed to load {:?}", cache.name);
    let mut theme = schema::Theme::from_file(&cache.path).expect(&error_message);

    let contains = theme.wallpapers.iter().any(|x| *x.path == wallpaper.path);

    match kind {
        ThemeWallpapers::TWAdd if !contains => theme.wallpapers.push(wallpaper),
        ThemeWallpapers::TWAdd => panic!("Current theme already has this wallpaper"),
        ThemeWallpapers::TWRemove => theme.wallpapers.retain(|x| *x.path != wallpaper.path),
    }

    theme.save();

    log::info!("Success");
}
