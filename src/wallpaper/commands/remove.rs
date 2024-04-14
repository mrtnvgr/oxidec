use crate::{config::Directory, state::schema, wallpaper::args};

pub fn handle(args: &args::Remove) {
    let name = args.filename.file_name().unwrap().to_str().unwrap();

    assert!(
        Directory::Wallpapers.contains(name),
        "This wallpaper does not exist"
    );

    for path in Directory::States.list() {
        let state = schema::State::from_file(&path).unwrap();
        let path = Directory::Wallpapers.get(name).unwrap();

        assert!(
            state.wallpaper.path != path,
            "\"{}\" state depends on this wallpaper",
            state.name
        );
    }

    match Directory::Wallpapers.remove(name) {
        Ok(()) => log::info!("Wallpaper was deleted successfully"),
        Err(error) => log::error!("Failed to delete a wallpaper: {}", error),
    }
}
