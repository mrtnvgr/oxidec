use crate::{
    cache::{self, status::Object},
    config::Folder,
    wallpaper::{args, set},
};
use std::path::Path;

pub fn handle(args: args::Set) {
    let name = get_wallpaper_name(args);
    ensure_that_wallpaper_exists(&name);
    ensure_that_path_is_filename(&name);

    let cache = cache::status::Wallpaper::new(&name, args.mode);
    cache.save().unwrap();

    let path = Folder::Wallpapers.get(&name).unwrap();
    set::wallpaper(path, args.mode);

    log::info!("Current wallpaper: {}", name);
}

fn get_wallpaper_name(args: args::Set) -> String {
    args.name
        .as_ref()
        .map_or_else(get_random_wallpaper, ToString::to_string)
}

fn get_random_wallpaper() -> String {
    let file = Folder::Wallpapers
        .random_file()
        .expect("There are no wallpapers.");
    let filename = file.file_name().unwrap();
    filename.to_str().unwrap().to_owned()
}

fn ensure_that_wallpaper_exists(name: &str) {
    assert!(
        Folder::Wallpapers.contains(name),
        "This wallpaper does not exist"
    );
}

fn ensure_that_path_is_filename(name: &str) {
    let path = Path::new(name);
    let filename = path.file_name().unwrap();
    assert!(
        path.as_os_str() == filename,
        "Wallpaper path should be a filename"
    );
}
