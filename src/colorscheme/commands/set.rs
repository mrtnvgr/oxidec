use crate::{
    cache::status::{Colorscheme, Object},
    colorscheme::{args, blocks, reloaders, schema, templates},
    config::Folder,
};

pub fn handle(args: &args::Set) {
    let name = args
        .name
        .clone()
        .unwrap_or_else(|| Folder::Colorschemes.random_entry());

    assert!(
        Folder::Colorschemes.contains(&name),
        "This colorscheme does not exist"
    );

    set(&name, args.gtk);

    log::info!("Current colorscheme: {}", name);

    blocks::print();
}

pub fn set(name: &str, gtk: bool) {
    let cache = Colorscheme::new(name);
    cache.save().unwrap();

    log::info!("Getting colorscheme...");
    let error_message = format!("Failed to load {name:?}");
    let colorscheme = schema::Colorscheme::from_file(&cache.path).expect(&error_message);

    log::info!("Generating templates...");
    templates::generate(&colorscheme);

    log::info!("Reloading colors...");
    reloaders::run(gtk);
}
