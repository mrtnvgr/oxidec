use crate::{
    cache::status::{Object, Wallpaper, WallpaperMode},
    config::Folder,
};

use std::{
    io,
    path::PathBuf,
    process::{Command, ExitStatus},
};
use which::which;

pub fn wallpaper(name: &str, mode: WallpaperMode) {
    let cache = Wallpaper::new(name, mode);
    cache.save().unwrap();

    let path = Folder::Wallpapers.get(name).unwrap();

    // TODO: support for desktops

    if which("feh").is_ok() {
        feh(path, mode).unwrap();
    } else if which("swaybg").is_ok() {
        swaybg(path, mode).unwrap();
    } else {
        log::error!("None of the supported wallpaper daemons are installed.");
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

fn swaybg(path: PathBuf, mode: WallpaperMode) -> io::Result<ExitStatus> {
    let mode = match mode {
        WallpaperMode::Center => "-m center",
        WallpaperMode::Fill => "-m fill",
        WallpaperMode::Max => "-m stretch",
        WallpaperMode::Scale => "-m fit",
        WallpaperMode::Tile => "-m tile",
        // TODO: WallpaperMode::Color => "-m solid_color",
    };

    Command::new("swaybg").arg(mode).arg(path).status()
}
