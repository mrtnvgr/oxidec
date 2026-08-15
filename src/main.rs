#[cfg(feature = "colorschemes")]
mod colorscheme;

#[cfg(feature = "wallpapers")]
mod wallpaper;

#[cfg(feature = "themes")]
mod theme;

mod cache;
mod common;
mod config;
mod logger;

use clap::Parser;

#[derive(Parser)]
struct Args {
    #[command(subcommand)]
    mode: Mode,
}

#[derive(Parser)]
enum Mode {
    #[cfg(feature = "colorschemes")]
    #[command(subcommand, name = "colorscheme", about = "colorscheme manager")]
    Colorscheme(colorscheme::args::Action),

    #[cfg(feature = "wallpapers")]
    #[command(subcommand, name = "wallpaper", about = "wallpaper manager")]
    Wallpaper(wallpaper::args::Action),

    #[cfg(feature = "themes")]
    #[command(subcommand, name = "theme", about = "theme manager")]
    Theme(theme::args::Action),
}

fn main() {
    logger::init();

    let args = Args::parse();
    config::ensure_config_exists();
    cache::ensure_cache_exists();

    match args.mode {
        #[cfg(feature = "colorschemes")]
        Mode::Colorscheme(args) => colorscheme::handle(args),
        #[cfg(feature = "wallpapers")]
        Mode::Wallpaper(args) => wallpaper::handle(args),
        #[cfg(feature = "themes")]
        Mode::Theme(args) => theme::handle(args),
    }
}
