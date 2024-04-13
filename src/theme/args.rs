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
    #[command(subcommand, name = "w", about = "theme wallpapers manager")]
    ThemeWallpapers(ThemeWallpapers),
}

// TODO: Add/remove wallpaper: th w(allpapers) (add/remove) (optional: specify wallpaper)
// TODO: Loop through wallpapers: th w rnd

// TODO: th rnd
// TODO: wl rnd
// TODO: cs rnd

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

#[derive(Subcommand)]
pub enum ThemeWallpapers {
    #[command(name = "add", about = "add current wallpaper to the theme")]
    TWAdd,
    #[command(name = "remove", about = "remove current wallpaper from the theme")]
    TWRemove,
}
