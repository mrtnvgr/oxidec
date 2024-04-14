use crate::cache::status::{Object, Wallpaper, WallpaperMode};

use std::{
    io,
    path::PathBuf,
    process::{Command, ExitStatus},
};
use which::which;

pub fn wallpaper(path: PathBuf, mode: WallpaperMode) {
    let cache = Wallpaper::new(path, mode);
    cache.save().unwrap();

    // TODO: support for desktops

    if which("feh").is_ok() {
        feh(cache).unwrap();
    } else if which("swaybg").is_ok() {
        swaybg(cache).unwrap();
    } else {
        log::error!("None of the supported wallpaper daemons are installed.");
    }
}

fn feh(wallpaper: Wallpaper) -> io::Result<ExitStatus> {
    let mode = match wallpaper.mode {
        WallpaperMode::Center => "--bg-center",
        WallpaperMode::Fill => "--bg-fill",
        WallpaperMode::Max => "--bg-max",
        WallpaperMode::Scale => "--bg-scale",
        WallpaperMode::Tile => "--bg-tile",
    };

    Command::new("feh").arg(mode).arg(wallpaper.path).status()
}

fn swaybg(wallpaper: Wallpaper) -> io::Result<ExitStatus> {
    let mode = match wallpaper.mode {
        WallpaperMode::Center => "-m center",
        WallpaperMode::Fill => "-m fill",
        WallpaperMode::Max => "-m stretch",
        WallpaperMode::Scale => "-m fit",
        WallpaperMode::Tile => "-m tile",
        // TODO: WallpaperMode::Color => "-m solid_color",
    };

    Command::new("swaybg")
        .arg(mode)
        .arg(wallpaper.path)
        .status()
}
