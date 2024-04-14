use crate::{config::Directory, state::args};

pub fn handle(args: &args::Remove) {
    assert!(
        Directory::States.contains(&args.name),
        "This state does not exist"
    );

    match Directory::States.remove(&args.name) {
        Ok(()) => log::info!("\"{}\" state was deleted successfully", args.name),
        Err(error) => log::error!("Failed to delete a state: {}", error),
    }
}
