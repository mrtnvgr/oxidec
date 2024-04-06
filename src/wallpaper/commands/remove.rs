use crate::{config::Folder, theme::schema, wallpaper::args};

pub fn handle(args: &args::Remove) {
    let name = args.filename.file_name().unwrap().to_str().unwrap();

    assert!(
        Folder::Wallpapers.contains(name),
        "This wallpaper does not exist"
    );

    for path in Folder::Themes.list() {
        let theme = schema::Theme::from_file(&path).unwrap();
        let path = Folder::Wallpapers.get(name).unwrap();

        assert!(
            theme.wallpaper.path != path,
            "\"{}\" theme depends on this wallpaper",
            theme.name
        );
    }

    match Folder::Wallpapers.remove(name) {
        Ok(()) => log::info!("Wallpaper was deleted successfully"),
        Err(error) => log::error!("Failed to delete a wallpaper: {}", error),
    }
}
