use crate::{
    cache::status::{self, Object, Wallpaper},
    theme::{
        args::{Switch, ThemeWallpapers},
        schema,
    },
    wallpaper,
};

use rand::prelude::*;

pub fn handle(kind: &ThemeWallpapers) {
    match kind {
        ThemeWallpapers::Add | ThemeWallpapers::Remove => toggle(kind),
        ThemeWallpapers::Switch(args) => set_wallpaper(args),
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
        ThemeWallpapers::Switch(_) => unreachable!(),
    }

    theme.save(&cache.name);

    log::info!("Success");
}

fn set_wallpaper(args: &Switch) {
    let wallpaper_cache = status::Wallpaper::load();

    let theme_cache = status::Theme::load();
    let error_message = format!("Failed to load {:?}", theme_cache.name);
    let theme = schema::Theme::from_file(&theme_cache.path).expect(&error_message);

    if args.random {
        set_random_wallpaper(&theme, &wallpaper_cache);
    } else {
        set_next_wallpaper(&theme, &wallpaper_cache);
    }
}

fn set_random_wallpaper(theme: &schema::Theme, wallpaper_cache: &Wallpaper) {
    loop {
        let wallpaper = get_random_wallpaper(theme);
        if wallpaper != wallpaper_cache {
            wallpaper::set(wallpaper.path.clone(), wallpaper.mode);
            break;
        }
    }
}

fn get_random_wallpaper(theme: &schema::Theme) -> &Wallpaper {
    let mut rng = thread_rng();
    theme.wallpapers.choose(&mut rng).unwrap()
}

fn set_next_wallpaper<'a>(
    theme: &'a schema::Theme,
    wallpaper_cache: &'a Wallpaper,
) -> &'a Wallpaper {
    let wallpaper_count = theme.wallpapers.len();
    let current_index = theme
        .wallpapers
        .iter()
        .position(|x| x == wallpaper_cache)
        .unwrap_or_default();

    let next_index = (current_index + 1).rem_euclid(wallpaper_count);

    #[allow(clippy::indexing_slicing)]
    &theme.wallpapers[next_index]
}
