use std::io::ErrorKind;

use crate::{colorscheme::args, config::Folder};

pub fn handle(args: &args::Remove) {
    match Folder::Colorschemes.remove(&args.name) {
        Ok(_) => log::info!("Colorscheme was deleted successfully"),
        Err(error) if error.kind() == ErrorKind::NotFound => {
            log::error!("This colorscheme does not exist");
        }
        Err(error) => log::error!("Failed to delete a colorscheme: {}", error),
    }
}
