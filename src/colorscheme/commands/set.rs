use crate::{
    cache::status::{Colorscheme, Object},
    colorscheme::{args, blocks, reloaders, schema, templates},
    config::Directory,
};

pub fn handle(args: &args::Set) {
    let name = args
        .name
        .clone()
        .unwrap_or_else(|| {
            if let Some(current) = Colorscheme::try_load() {
                Directory::Colorschemes.random_entry_excluding(&current.name)
            } else {
                Directory::Colorschemes.random_entry()
            }
        });

    log::debug!("Getting colorscheme...");

    let colorscheme_path = Directory::Colorschemes
        .get(&name)
        .expect("This colorscheme does not exist");

    let error_message = format!("Failed to load {name:?}");
    let colorscheme = schema::Colorscheme::from_file(&colorscheme_path).expect(&error_message);

    let cache = Colorscheme::new(&name);
    cache.save().unwrap();

    set_without_cache(&colorscheme);

    log::debug!("Current colorscheme: {name}");

    blocks::print();
}

pub fn set_without_cache(colorscheme: &schema::Colorscheme) {
    log::debug!("Generating templates...");
    templates::generate(colorscheme);

    log::debug!("Reloading colors...");
    reloaders::run();
}
