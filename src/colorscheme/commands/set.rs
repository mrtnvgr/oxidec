use crate::{
    cache::status::{Colorscheme, Object},
    colorscheme::{args, blocks, reloaders, schema, templates},
    config::Folder,
};
use std::{path::PathBuf, string::ToString};

pub fn handle(args: &args::Set) {
    let name = get_colorscheme_name(args);
    ensure_that_colorscheme_exists(&name);

    let cache = Colorscheme::new(&name);
    cache.save().unwrap();

    log::info!("Getting colorscheme...");
    let colorscheme = load_colorscheme(&cache.path);

    log::info!("Generating templates...");
    templates::generate(&colorscheme);

    log::info!("Reloading colors...");
    reloaders::run(args.gtk);

    log::info!("Current colorscheme: {}", name);

    blocks::print();
}

fn get_colorscheme_name(args: &args::Set) -> String {
    args.name
        .as_ref()
        .map_or_else(get_random_colorscheme, ToString::to_string)
}

fn get_random_colorscheme() -> String {
    let file = Folder::Colorschemes
        .random_file()
        .expect("There are no colorschemes.");
    let filestem = file.file_stem().unwrap();
    filestem.to_str().unwrap().to_owned()
}

fn ensure_that_colorscheme_exists(name: &str) {
    assert!(
        Folder::Colorschemes.contains(name),
        "This colorscheme does not exist"
    );
}

fn load_colorscheme(path: &PathBuf) -> schema::Colorscheme {
    let binding = path.with_extension("");
    let colorscheme = binding.file_name().unwrap();

    let error_message = format!("Failed to load {colorscheme:?}");
    schema::Colorscheme::from_file(path).expect(&error_message)
}
