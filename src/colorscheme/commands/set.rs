use crate::config::Directory;
use crate::colorscheme::{args, blocks, reloaders, schema, templates};
use crate::cache::status::{Colorscheme, Object};

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

    let status = Colorscheme::new(&name);
    status.save();

    log::info!("Current colorscheme: {name}");

    let colorscheme = schema::Colorscheme::from_status(status);
    set_without_cache(&colorscheme);

    blocks::print();
}

pub fn set_without_cache(colorscheme: &schema::Colorscheme) {
    log::debug!("Generating templates...");
    templates::generate(colorscheme);

    log::debug!("Reloading colors...");
    reloaders::run();
}
