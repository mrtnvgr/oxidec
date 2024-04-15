use crate::{config::Directory, theme::schema, wallpaper::args};

pub fn handle(args: &args::Remove) {
    let name = args.filename.file_name().unwrap().to_str().unwrap();

    assert!(
        Directory::Wallpapers.contains(name),
        "This wallpaper does not exist"
    );

    for path in Directory::Themes.list() {
        let theme = schema::Theme::from_file(&path).unwrap();
        let theme_name = path.file_name().unwrap().to_string_lossy();
        let path = Directory::Wallpapers.get(name).unwrap();

        for wallpaper in theme.wallpapers {
            assert!(
                wallpaper.path != path,
                "\"{}\" theme depends on this wallpaper",
                theme_name
            );
        }
    }

    match Directory::Wallpapers.remove(name) {
        Ok(()) => log::info!("Wallpaper was deleted successfully"),
        Err(error) => log::error!("Failed to delete a wallpaper: {}", error),
    }
}
