use crate::{config::Directory, wallpaper::args};

pub fn handle(args: &args::List) {
    if args.json {
        output_using_json();
    } else {
        output_using_log();
    }
}

fn output_using_log() {
    log::info!("Wallpapers: ");
    for entry in Directory::Wallpapers.list_names() {
        log::info!("{:?}", entry);
    }
}

fn output_using_json() {
    let list = Directory::Wallpapers.list_names();
    print!("{}", serde_json::to_string(&list).unwrap());
}
