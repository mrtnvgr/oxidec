use crate::{config::Directory, wallpaper::args};
use file_format::{FileFormat, Kind};

pub fn handle(args: &args::Add) {
    assert!(args.file_path.exists(), "This file does not exist");

    let format = FileFormat::from_file(&args.file_path).unwrap();
    assert!(format.kind() == Kind::Image, "This file is not an image");

    let filename = args.file_path.file_name().unwrap();
    let name = filename.to_str().unwrap();
    assert!(
        !Directory::Wallpapers.contains(name),
        "Wallpaper with this name already exists!"
    );

    Directory::Wallpapers.copy(&args.file_path).unwrap();
    log::info!("Added successfully!");
}
