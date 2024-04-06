use crate::{
    cache::status::{Colorscheme, Object},
    colorscheme::args,
};

pub fn handle(args: &args::Status) {
    let status = Colorscheme::load();

    if args.json {
        print!("{}", serde_json::to_string(&status).unwrap());
    } else {
        log::info!("Name: {}", status.name);
        log::info!("Path: {}", status.path.display());
    }
}
