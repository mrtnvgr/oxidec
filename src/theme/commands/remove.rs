use crate::{config::Directory, theme::args};

pub fn handle(args: &args::Remove) {
    assert!(
        Directory::Themes.contains(&args.name),
        "This theme does not exist"
    );

    match Directory::Themes.remove(&args.name) {
        Ok(()) => log::info!("\"{}\" theme was deleted successfully", args.name),
        Err(error) => log::error!("Failed to delete a theme: {}", error),
    }
}
