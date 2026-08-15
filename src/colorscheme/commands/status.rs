use crate::common;
use crate::{
    cache::status::{Colorscheme, Object},
    colorscheme::args,
};

pub fn handle(args: &args::Status) {
    let status = Colorscheme::load();

    if args.json {
        common::print_json(&status);
    } else {
        log::info!("Name: {}", status.name);
        log::info!("Path: {}", status.path.display());
    }
}
