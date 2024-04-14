use crate::{
    colorscheme,
    config::Folder,
    state::{args, schema},
    wallpaper,
};

pub fn handle(args: &args::Set) {
    let name = args
        .name
        .clone()
        .unwrap_or_else(|| Folder::States.random_entry());

    let path = Folder::States
        .get(&name)
        .expect("This state does not exist");

    let error_message = format!("Failed to load {name:?}");
    let state = schema::State::from_file(&path).expect(&error_message);

    colorscheme::set(&state.colorscheme.name, false);

    let wallpaper = state.wallpaper.path;
    let wallpaper_name = wallpaper.file_name().unwrap().to_string_lossy();

    wallpaper::set(wallpaper.clone(), state.wallpaper.mode);

    log::info!("Current colorscheme: {}", state.colorscheme.name);
    log::info!("Current wallpaper: {}", wallpaper_name);

    colorscheme::blocks::print();
}
