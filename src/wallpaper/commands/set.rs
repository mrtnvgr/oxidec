use crate::{
    config::Folder,
    wallpaper::{self, args},
};
use std::path::Path;

pub fn handle(args: args::Set) {
    let name = args.name.unwrap_or_else(get_random_wallpaper);
    ensure_that_wallpaper_exists(&name);
    ensure_that_path_is_filename(&name);

    wallpaper::set(&name, args.mode);

    log::info!("Current wallpaper: {}", name);
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
