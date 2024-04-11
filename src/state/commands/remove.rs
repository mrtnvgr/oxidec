use crate::{config::Folder, state::args};

pub fn handle(args: args::Remove) {
    assert!(
        Folder::States.contains(&args.name),
        "This state does not exist"
    );

    match Folder::States.remove(&args.name) {
        Ok(()) => log::info!("\"{}\" state was deleted successfully", args.name),
        Err(error) => log::error!("Failed to delete a state: {}", error),
    }
}
