use crate::cache::status::{Object, Wallpaper, WallpaperMode};

use std::io;
use std::path::PathBuf;
use std::process::{Command, ExitStatus};
use which::which;

pub fn wallpaper(path: PathBuf, mode: WallpaperMode) {
    let cache = Wallpaper { path, mode };
    cache.save();

    // TODO: support for DEs

    if which("feh").is_ok() {
        feh(cache).unwrap();
    } else if which("swaybg").is_ok() {
        swaybg(cache).unwrap();
    } else {
        log::error!("None of the supported wallpaper daemons are installed.");
    }
}

fn feh(wallpaper: Wallpaper) -> io::Result<ExitStatus> {
    Command::new("feh")
        .arg(wallpaper.mode.feh_flag())
        .arg(wallpaper.path)
        .status()
}

fn swaybg(wallpaper: Wallpaper) -> io::Result<ExitStatus> {
    // kill all other instances
    kill_all("swaybg");

    Command::new("swaybg")
        .arg(wallpaper.mode.swaybg_flag())
        .arg(wallpaper.path)
        .status()
}

impl WallpaperMode {
    fn feh_flag(&self) -> &'static str {
        match self {
            WallpaperMode::Center => "--bg-center",
            WallpaperMode::Fill => "--bg-fill",
            WallpaperMode::Max => "--bg-max",
            WallpaperMode::Scale => "--bg-scale",
            WallpaperMode::Tile => "--bg-tile",
        }
    }

    fn swaybg_flag(&self) -> &'static str {
        match self {
            WallpaperMode::Center => "-m center",
            WallpaperMode::Fill => "-m fill",
            WallpaperMode::Max => "-m stretch",
            WallpaperMode::Scale => "-m fit",
            WallpaperMode::Tile => "-m tile",
            // TODO: WallpaperMode::Color => "-m solid_color",
        }
    }
}

fn kill_all(proc_name: &str) {
    let _ = Command::new("pkill")
        .arg("-9")
        .arg("-x")
        .arg(proc_name)
        .status();
}
