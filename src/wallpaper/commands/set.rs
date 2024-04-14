use crate::{
    config::Directory,
    wallpaper::{self, args},
};
use std::path::Path;

pub fn handle(args: &args::Set) {
    let name = args
        .name
        .clone()
        .unwrap_or_else(|| Directory::Wallpapers.random_entry());

    ensure_that_path_is_a_filename(&name);

    let path = Directory::Wallpapers
        .get(&name)
        .expect("This wallpaper does not exist");

    wallpaper::set(path, args.mode);

    log::info!("Current wallpaper: {}", name);
}

fn ensure_that_path_is_a_filename(name: &str) {
    let path = Path::new(name);
    let filename = path.file_name().unwrap();
    assert!(
        path.as_os_str() == filename,
        "Wallpaper path should be a filename"
    );
}
