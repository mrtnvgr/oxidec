mod colorscheme;
mod state;
mod wallpaper;

mod cache;
mod config;

use clap::Parser;

#[derive(Parser)]
struct Args {
    #[command(subcommand)]
    mode: Mode,
}

#[derive(Parser)]
enum Mode {
    #[command(subcommand, name = "colorscheme", about = "colorscheme manager")]
    Colorscheme(colorscheme::args::Action),
    #[command(subcommand, name = "wallpaper", about = "wallpaper manager")]
    Wallpaper(wallpaper::args::Action),
    #[command(subcommand, name = "state", about = "state manager")]
    State(state::args::Action),
}

fn main() {
    cli_logger::init();
    let args = Args::parse();
    config::ensure_config_exists();
    cache::ensure_cache_exists();

    match args.mode {
        Mode::Colorscheme(args) => colorscheme::handle(args),
        Mode::Wallpaper(args) => wallpaper::handle(args),
        Mode::State(args) => state::handle(args),
        Mode::Theme(args) => theme::handle(args),
    }
}
