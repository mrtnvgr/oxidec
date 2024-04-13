use crate::{
    colorscheme,
    config::Folder,
    state::{args, schema},
    wallpaper,
};

pub fn handle(args: &args::Set) {
    let name = args.name.clone().unwrap_or_else(get_random_state);
    ensure_that_state_exists(&name);

    let path = Folder::States.get(&name).unwrap();

    let error_message = format!("Failed to load {name:?}");
    let state = schema::State::from_file(&path).expect(&error_message);

    colorscheme::set(&state.colorscheme.name, false);
    wallpaper::set(&state.wallpaper.name, state.wallpaper.mode);

    log::info!("Current colorscheme: {}", state.colorscheme.name);
    log::info!("Current wallpaper: {}", state.wallpaper.name);

    colorscheme::blocks::print();
}

fn get_random_state() -> String {
    let file = Folder::States.random_file().expect("There are no states.");
    let filestem = file.file_stem().unwrap();
    filestem.to_str().unwrap().to_owned()
}

fn ensure_that_state_exists(name: &str) {
    assert!(Folder::States.contains(name), "This state does not exist");
}
