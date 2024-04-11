use crate::{
    cache::status::{self, Object},
    config::Folder,
    state::{args, schema},
};
use std::fs::File;

pub fn handle(args: args::Save) {
    assert!(
        !Folder::States.contains(&args.name),
        "A state with this name already exists"
    );

    let colorscheme = status::Colorscheme::load();
    let wallpaper = status::Wallpaper::load();

    let state = schema::State::new(&args.name, colorscheme, wallpaper);

    let path = Folder::States.build_path(&args.name);
    serde_json::to_writer(File::create(path).unwrap(), &state).unwrap();

    log::info!("Saved successfully");
}
