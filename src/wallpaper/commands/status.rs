use crate::common;
use crate::{
    cache::status::{self, Object},
    wallpaper::args,
};

pub fn handle(args: &args::Status) {
    let status = status::Wallpaper::load();

    if args.json {
        common::print_json(&status);
    } else {
        log::info!("Path: {}", status.path.display());
        log::info!("Mode: {:?}", status.mode);
    }
}
