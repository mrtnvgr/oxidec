use crate::{colorscheme::args, config::Directory, state::schema};

pub fn handle(args: &args::Remove) {
    assert!(
        Directory::Colorschemes.contains(&args.name),
        "This colorscheme does not exist"
    );

    for path in Directory::States.list() {
        let state = schema::State::from_file(&path).unwrap();
        let path = Directory::Colorschemes.get(&args.name).unwrap();

        assert!(
            state.colorscheme.path != path,
            "\"{}\" state depends on this colorscheme",
            state.name
        );
    }

    match Directory::Colorschemes.remove(&args.name) {
        Ok(()) => log::info!("\"{}\" colorscheme was deleted successfully", args.name),
        Err(error) => log::error!("Failed to delete a colorscheme: {}", error),
    }
}
