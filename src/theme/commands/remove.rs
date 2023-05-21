use crate::{config::Folder, theme::args};
use std::io::ErrorKind;

pub fn handle(args: &args::Remove) {
    match Folder::Themes.remove(&args.name) {
        Ok(_) => log::info!("Theme was deleted successfully"),
        Err(error) if error.kind() == ErrorKind::NotFound => {
            log::error!("This theme does not exist");
        }
        Err(error) => log::error!("Failed to delete a theme: {}", error),
    }
}
