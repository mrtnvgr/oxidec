use crate::{
    cache::status::{Object, Wallpaper, WallpaperMode},
    config::Folder,
};

use std::{
    io,
    path::PathBuf,
    process::{Command, ExitStatus},
};

pub fn wallpaper(name: &str, mode: WallpaperMode) {
    let cache = Wallpaper::new(&name, mode);
    cache.save().unwrap();

    let path = Folder::Wallpapers.get(name).unwrap();

    // TODO: support desktops, wayland
    if feh(path, mode).is_err() {
        log::error!("Feh is not installed on your system.");
    }
}

fn feh(path: PathBuf, mode: WallpaperMode) -> io::Result<ExitStatus> {
    let mode = match mode {
        WallpaperMode::Center => "--bg-center",
        WallpaperMode::Fill => "--bg-fill",
        WallpaperMode::Max => "--bg-max",
        WallpaperMode::Scale => "--bg-scale",
        WallpaperMode::Tile => "--bg-tile",
    };

    Command::new("feh").arg(mode).arg(path).status()
}
