use crate::{
    cache::status::{self, Object},
    wallpaper::args,
};

pub fn handle(args: args::Status) {
    let status = status::Wallpaper::load();

    if args.json {
        print!("{}", serde_json::to_string(&status).unwrap());
    } else {
        log::info!("Name: {}", status.name);
        log::info!("Path: {}", status.path.display());
        log::info!("Mode: {:?}", status.mode);
    }
}
