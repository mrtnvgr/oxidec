mod colorscheme;
mod wallpaper;

pub mod cache;
pub mod config;
mod logger;

use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "oxidec", about = "Eye-candy manager written in Rust")]
struct OxidecArgs {
    #[structopt(subcommand)]
    mode: Mode,
}

#[derive(StructOpt)]
enum Mode {
    #[structopt(name = "colorscheme", about = "colorscheme manager")]
    Colorscheme(colorscheme::args::Action),
    #[structopt(name = "wallpaper", about = "wallpaper manager")]
    Wallpaper(wallpaper::args::Action),
}

fn main() {
    logger::init();
    let args = OxidecArgs::from_args();
    config::ensure_config_exists();
    cache::ensure_cache_exists();

    match args.mode {
        Mode::Colorscheme(args) => colorscheme::handle(&args),
        Mode::Wallpaper(args) => wallpaper::handle(&args),
    }
}
