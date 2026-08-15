use crate::cache::status::{Colorscheme, Object};
use crate::colorscheme::{args, blocks, reloaders, schema, templates};
use crate::config::Directory;

pub fn handle(args: &args::Set) {
    let name = args.name.clone().unwrap_or_else(|| {
        let current = Colorscheme::try_load().map(|status| status.name);
        Directory::Colorschemes.pick_random(current)
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
