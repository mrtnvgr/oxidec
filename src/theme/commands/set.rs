use crate::{
    colorscheme,
    config::Folder,
    theme::{args, schema},
    wallpaper,
};

pub fn handle(args: args::Set) {
    let name = args.name.unwrap_or_else(get_random_theme);

    assert!(Folder::Themes.contains(&name), "This theme does not exist");

    let path = Folder::Themes.get(&name).unwrap();

    let error_message = format!("Failed to load {name:?}");
    let mut theme = schema::Theme::from_file(&path).expect(&error_message);

    colorscheme::set(&theme.colorscheme.name, false);

    // TODO: get random wallpaper
    let wallpaper = theme.wallpapers.pop().unwrap();
    wallpaper::set(&wallpaper.name, wallpaper.mode);

    log::info!("Current colorscheme: {}", theme.colorscheme.name);
    log::info!("Current wallpaper: {}", wallpaper.name);

    colorscheme::blocks::print();
}

fn get_random_theme() -> String {
    let file = Folder::Themes.random_file().expect("There are no themes.");
    let filestem = file.file_stem().unwrap();
    filestem.to_str().unwrap().to_owned()
}
