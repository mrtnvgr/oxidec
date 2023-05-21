use crate::cache::status::WallpaperMode;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
pub enum Action {
    #[structopt(name = "add", about = "<filename> add wallpaper")]
    Add(Add),
    #[structopt(name = "set", about = "[filename] set wallpaper")]
    Set(Set),
    #[structopt(name = "remove", about = "<filename> remove wallpaper")]
    Remove(Remove),
    #[structopt(name = "list", about = "list wallpapers")]
    List(List),
    #[structopt(name = "status", about = "wallpaper status")]
    Status(Status),
}

#[derive(StructOpt)]
pub struct Add {
    #[structopt(parse(from_os_str))]
    pub file_path: PathBuf,
}

#[derive(StructOpt)]
pub struct Set {
    pub name: Option<String>,
    #[structopt(long = "mode", default_value = "fill", case_insensitive = true, possible_values = &WallpaperMode::variants())]
    pub mode: WallpaperMode,
}

#[derive(StructOpt)]
pub struct Remove {
    #[structopt(parse(from_os_str))]
    pub filename: PathBuf,
}

#[derive(StructOpt)]
pub struct List {
    #[structopt(long)]
    pub json: bool,
}

#[derive(StructOpt)]
pub struct Status {
    #[structopt(long)]
    pub json: bool,
}
