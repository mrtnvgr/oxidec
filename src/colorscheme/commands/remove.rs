use crate::{colorscheme::args, config::Folder, state::schema};

pub fn handle(args: args::Remove) {
    assert!(
        Folder::Colorschemes.contains(&args.name),
        "This colorscheme does not exist"
    );

    for path in Folder::States.list() {
        let state = schema::State::from_file(&path).unwrap();
        let path = Folder::Colorschemes.get(&args.name).unwrap();

        assert!(
            state.colorscheme.path != path,
            "\"{}\" state depends on this colorscheme",
            state.name
        );
    }

    match Folder::Colorschemes.remove(&args.name) {
        Ok(()) => log::info!("\"{}\" colorscheme was deleted successfully", args.name),
        Err(error) => log::error!("Failed to delete a colorscheme: {}", error),
    }
}
