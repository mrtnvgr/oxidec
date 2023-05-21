use crate::cache::status::WallpaperMode;
use std::{
    io,
    path::PathBuf,
    process::{Command, ExitStatus},
};

pub fn wallpaper(path: PathBuf, mode: WallpaperMode) {
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
