use crate::{config::Folder, theme::args};

pub fn handle(args: &args::Remove) {
    assert!(Folder::Themes.contains(&args.name), "This theme does not exist");

    match Folder::Themes.remove(&args.name) {
        Ok(_) => log::info!("Theme was deleted successfully"),
        Err(error) => log::error!("Failed to delete a theme: {}", error),
    }
}
