use crate::{config::Folder, wallpaper::args};

pub fn handle(args: &args::List) {
    if args.json {
        output_using_json();
    } else {
        output_using_log();
    }
}

fn output_using_log() {
    log::info!("Wallpapers: ");
    for entry in Folder::Wallpapers.list().unwrap() {
        let wallpaper = entry.file_name().unwrap();
        log::info!("{:?}", wallpaper);
    }
}

fn output_using_json() {
    let list = Folder::Wallpapers.list().unwrap();
    print!("{}", serde_json::to_string(&list).unwrap());
}
