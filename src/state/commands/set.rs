use crate::{
    cache::status::Object,
    colorscheme,
    config::Folder,
    state::{args, schema},
    wallpaper,
};
use std::path::PathBuf;

pub fn handle(args: &args::Set) {
    let name = get_state_name(args);
    ensure_that_state_exists(&name);

    let path = Folder::States.get(&name).unwrap();
    let state = load_state(&path);

    state.colorscheme.save().unwrap();
    state.wallpaper.save().unwrap();

    log::info!("Generating templates...");
    let error_message = format!("Failed to load {:?}", state.colorscheme.name);
    let path = state.colorscheme.path;
    let colorscheme = colorscheme::schema::Colorscheme::from_file(&path).expect(&error_message);
    colorscheme::templates::generate(&colorscheme);

    log::info!("Reloading colors...");
    colorscheme::reloaders::run(false);

    log::info!("Setting wallpaper...");
    wallpaper::set::wallpaper(state.wallpaper.path, state.wallpaper.mode);

    log::info!("Current colorscheme: {}", state.colorscheme.name);
    log::info!("Current wallpaper: {}", state.wallpaper.name);

    colorscheme::blocks::print();
}

fn get_state_name(args: &args::Set) -> String {
    args.name
        .as_ref()
        .map_or_else(get_random_state, ToString::to_string)
}

fn get_random_state() -> String {
    let file = Folder::States.random_file().expect("There are no states.");
    let filestem = file.file_stem().unwrap();
    filestem.to_str().unwrap().to_owned()
}

fn ensure_that_state_exists(name: &str) {
    assert!(Folder::States.contains(name), "This state does not exist");
}

fn load_state(path: &PathBuf) -> schema::State {
    let binding = path.with_extension("");
    let state = binding.file_name().unwrap();

    let error_message = format!("Failed to load {state:?}");
    schema::State::from_file(path).expect(&error_message)
}
