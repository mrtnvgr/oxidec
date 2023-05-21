use crate::{
    colorscheme,
    config::Folder,
    theme::{args, schema},
    wallpaper,
};
use std::path::PathBuf;

pub fn handle(args: &args::Set) {
    let name = get_theme_name(args);
    ensure_that_theme_exists(&name);

    let path = Folder::Themes.get(&name).unwrap();
    let theme = load_theme(&path);

    theme.colorscheme.save().unwrap();
    theme.wallpaper.save().unwrap();

    log::info!("Generating templates...");
    let error_message = format!("Failed to load {:?}", theme.colorscheme.name);
    let path = theme.colorscheme.path;
    let colorscheme = colorscheme::schema::Colorscheme::from_file(&path).expect(&error_message);
    colorscheme::templates::generate(&colorscheme);

    log::info!("Reloading colors...");
    colorscheme::reloaders::run();

    log::info!("Setting wallpaper...");
    wallpaper::set::wallpaper(theme.wallpaper.path, theme.wallpaper.mode);

    log::info!("Current colorscheme: {}", theme.colorscheme.name);
    log::info!("Current wallpaper: {}", theme.wallpaper.name);

    colorscheme::blocks::print();
}

fn get_theme_name(args: &args::Set) -> String {
    args.name
        .as_ref()
        .map_or_else(get_random_theme, ToString::to_string)
}

fn get_random_theme() -> String {
    let file = Folder::Themes.random_file().expect("There are no themes.");
    let filestem = file.file_stem().unwrap();
    filestem.to_str().unwrap().to_owned()
}

fn ensure_that_theme_exists(name: &str) {
    assert!(Folder::Themes.contains(name), "This theme does not exist");
}

fn load_theme(path: &PathBuf) -> schema::Theme {
    let binding = path.with_extension("");
    let theme = binding.file_name().unwrap();

    let error_message = format!("Failed to load {theme:?}");
    schema::Theme::from_file(path).expect(&error_message)
}
