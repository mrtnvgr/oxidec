use crate::{config::Directory, wallpaper::args};
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub fn handle(args: &args::Add) {
    assert!(args.file_path.exists(), "This file does not exist");
    assert!(is_image(&args.file_path), "This file is not an image");

    let filename = args.file_path.file_name().unwrap();
    let name = filename.to_str().unwrap();
    assert!(
        !Directory::Wallpapers.contains(name),
        "Wallpaper with this name already exists!"
    );

    Directory::Wallpapers.copy(&args.file_path).unwrap();
    log::info!("Added successfully!");
}

fn is_image(path: &Path) -> bool {
    let mut magic = [0u8; 16];
    let mut file = File::open(path).unwrap();
    if file.read_exact(&mut magic).is_err() {
        return false;
    }

    let png = magic.starts_with(b"\x89PNG\r\n\x1a\n");
    let jpeg = magic.starts_with(b"\xFF\xD8\xFF");
    let bmp = magic.starts_with(b"BM");

    png || jpeg || bmp
}
