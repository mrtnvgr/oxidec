use clap::{Args, Subcommand};

#[derive(Subcommand)]
pub enum Action {
    #[command(name = "new", about = "<name> create a theme")]
    New(New),
    #[command(name = "set", about = "[name] set theme")]
    Set(Set),
    #[command(name = "remove", about = "<name> remove theme")]
    Remove(Remove),
    #[command(name = "list", about = "list themes")]
    List(List),
}

// TODO: Add/remove wallpaper: th w(allpapers) (add/remove) (optional: specify wallpaper)
// TODO: Loop through wallpapers: th w(allpapers) next, th wallpapers previous, th wallpapers random

#[derive(Args)]
pub struct Set {
    pub name: Option<String>,
}

#[derive(Args)]
pub struct New {
    pub name: String,
}

#[derive(Args)]
pub struct Remove {
    pub name: String,
}

#[derive(Args)]
pub struct List {
    #[arg(long)]
    pub json: bool,
}
