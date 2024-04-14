use crate::{
    cache::status::{self, Object},
    config::Directory,
    state::{args, schema},
};

pub fn handle(args: &args::Save) {
    assert!(
        !Directory::States.contains(&args.name),
        "A state with this name already exists"
    );

    let colorscheme = status::Colorscheme::load();
    let wallpaper = status::Wallpaper::load();

    let state = schema::State::new(&args.name, colorscheme, wallpaper);
    state.save();

    log::info!("Saved as {}", args.name);
}
