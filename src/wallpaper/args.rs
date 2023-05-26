use crate::cache::status::WallpaperMode;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Subcommand)]
pub enum Action {
    #[command(name = "add", about = "<filename> add wallpaper")]
    Add(Add),
    #[command(name = "set", about = "[filename] set wallpaper")]
    Set(Set),
    #[command(name = "remove", about = "<filename> remove wallpaper")]
    Remove(Remove),
    #[command(name = "list", about = "list wallpapers")]
    List(List),
    #[command(name = "status", about = "wallpaper status")]
    Status(Status),
}

#[derive(Parser)]
pub struct Add {
    pub file_path: PathBuf,
}

#[derive(Parser)]
pub struct Set {
    pub name: Option<String>,
    #[arg(long = "mode", default_value = "fill", ignore_case = true)]
    pub mode: WallpaperMode,
}

#[derive(Parser)]
pub struct Remove {
    pub filename: PathBuf,
}

#[derive(Parser)]
pub struct List {
    #[arg(long)]
    pub json: bool,
}

#[derive(Parser)]
pub struct Status {
    #[arg(long)]
    pub json: bool,
}
