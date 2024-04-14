use crate::{colorscheme::args, config::Directory};

pub fn handle(args: &args::Remove) {
    assert!(
        Directory::Colorschemes.contains(&args.name),
        "This colorscheme does not exist"
    );

    match Directory::Colorschemes.remove(&args.name) {
        Ok(()) => log::info!("\"{}\" colorscheme was deleted successfully", args.name),
        Err(error) => log::error!("Failed to delete a colorscheme: {}", error),
    }
}
