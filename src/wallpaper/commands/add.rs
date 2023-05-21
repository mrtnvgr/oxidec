use crate::{config::Folder, wallpaper::args};

pub fn handle(args: &args::Add) {
    // TODO: check if file is a photo
    assert!(args.file_path.exists(), "This file does not exist");

    let filename = args.file_path.file_name().unwrap();
    let name = filename.to_str().unwrap();
    assert!(
        !Folder::Wallpapers.contains(name),
        "Wallpaper with this name already exists!"
    );

    Folder::Wallpapers.copy(&args.file_path).unwrap();
    log::info!("Added successfully!");
}
