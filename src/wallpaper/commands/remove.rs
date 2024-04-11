use crate::{config::Folder, state::schema, wallpaper::args};

pub fn handle(args: args::Remove) {
    let name = args.filename.file_name().unwrap().to_str().unwrap();

    assert!(
        Folder::Wallpapers.contains(name),
        "This wallpaper does not exist"
    );

    for path in Folder::States.list() {
        let state = schema::State::from_file(&path).unwrap();
        let path = Folder::Wallpapers.get(name).unwrap();

        assert!(
            state.wallpaper.path != path,
            "\"{}\" state depends on this wallpaper",
            state.name
        );
    }

    match Folder::Wallpapers.remove(name) {
        Ok(()) => log::info!("Wallpaper was deleted successfully"),
        Err(error) => log::error!("Failed to delete a wallpaper: {}", error),
    }
}
