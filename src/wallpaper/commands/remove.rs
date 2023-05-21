use crate::{config::Folder, wallpaper::args};
use std::io::ErrorKind;

pub fn handle(args: &args::Remove) {
    let name = args.filename.file_name().unwrap().to_str().unwrap();
    match Folder::Wallpapers.remove(name) {
        Ok(_) => log::info!("Wallpaper was deleted successfully"),
        Err(error) if error.kind() == ErrorKind::NotFound => {
            log::error!("This wallpaper does not exist");
        }
        Err(error) => log::error!("Failed to delete a wallpaper: {}", error),
    }
}
