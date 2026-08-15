use crate::{config::Directory, wallpaper::args};

#[cfg(feature = "themes")]
use crate::theme::schema;

pub fn handle(args: &args::Remove) {
    let name = args.filename.file_name().unwrap().to_str().unwrap();

    assert!(
        Directory::Wallpapers.contains(name),
        "This wallpaper does not exist"
    );

    #[cfg(feature = "themes")]
    check_theme_dep(name);

    match Directory::Wallpapers.remove(name) {
        Ok(()) => log::info!("Wallpaper was deleted successfully"),
        Err(error) => log::error!("Failed to delete a wallpaper: {error}"),
    }
}

#[cfg(feature = "themes")]
fn check_theme_dep(name: &str) {
    let path = Directory::Wallpapers.get(name).unwrap();

    for theme_path in Directory::Themes.list() {
        let theme = schema::Theme::from_file(&theme_path).unwrap();
        let theme_name = theme_path.file_name().unwrap().to_string_lossy();

        for wallpaper in theme.wallpapers {
            assert!(
                wallpaper.path != path,
                "\"{theme_name}\" theme depends on this wallpaper",
            );
        }
    }
}
