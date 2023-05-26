use super::commands::generate::backends::Backend;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Subcommand)]
pub enum Action {
    #[command(name = "set", about = "[name] set colorscheme")]
    Set(Set),
    #[command(name = "remove", about = "<name> remove colorscheme")]
    Remove(Remove),
    #[command(name = "import", about = "<file-path> import colorscheme")]
    Import(Import),
    #[command(name = "list", about = "list colorschemes")]
    List(List),
    #[command(name = "status", about = "colorscheme status")]
    Status(Status),
    #[command(name = "generate", about = "generate colorscheme from wallpaper")]
    Generate(Generate),
    #[command(name = "reload", about = "reload colorscheme")]
    Reload,
}

#[derive(Parser)]
pub struct Set {
    pub name: Option<String>,
}

#[derive(Parser)]
pub struct Remove {
    pub name: String,
}

#[derive(Parser)]
pub struct Import {
    pub file_path: PathBuf,
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

#[derive(Parser)]
pub struct Generate {
    #[arg(value_enum, long, default_value = "imagemagick", ignore_case = true)]
    pub backend: Backend,
    #[arg(long)]
    pub light: bool,
}
