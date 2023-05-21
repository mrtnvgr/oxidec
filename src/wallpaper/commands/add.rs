use crate::{config::Folder, wallpaper::args};

pub fn handle(args: &args::Add) {
    // TODO: check if file is a photo
    assert!(args.file_path.exists(), "This file does not exist");
    Folder::Wallpapers.copy(&args.file_path).unwrap();
    log::info!("Added successfully!");
}
