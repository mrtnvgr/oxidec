mod colorscheme;
mod theme;
mod wallpaper;

pub mod cache;
pub mod config;

use clap::Parser;

#[derive(Parser)]
#[command(author, version, about)]
struct OxidecArgs {
    #[command(subcommand)]
    mode: Mode,
}

#[derive(Parser)]
enum Mode {
    #[command(subcommand, name = "colorscheme", about = "colorscheme manager")]
    Colorscheme(colorscheme::args::Action),
    #[command(subcommand, name = "wallpaper", about = "wallpaper manager")]
    Wallpaper(wallpaper::args::Action),
    #[command(subcommand, name = "theme", about = "theme manager")]
    Theme(theme::args::Action),
}

fn main() {
    cli_logger::init();
    let args = OxidecArgs::parse();
    config::ensure_config_exists();
    cache::ensure_cache_exists();

    match args.mode {
        Mode::Colorscheme(args) => colorscheme::handle(&args),
        Mode::Wallpaper(args) => wallpaper::handle(&args),
        Mode::Theme(args) => theme::handle(&args),
    }
}
