use crate::{
    cache::status::{self, Object},
    colorscheme,
    config::Directory,
    theme::{args, schema},
};

use std::fs::File;

pub fn handle(args: &args::New) {
    assert!(
        !Directory::Themes.contains(&args.name),
        "A theme with this name already exists"
    );

    let colorscheme = status::Colorscheme::load();
    let colorscheme = colorscheme::schema::Colorscheme::from_file(&colorscheme.path).unwrap();

    let wallpaper = status::Wallpaper::load();

    let theme = schema::Theme::new(colorscheme, vec![wallpaper]);

    let path = Directory::Themes.build_path(&args.name);
    serde_json::to_writer(File::create(path).unwrap(), &theme).unwrap();

    let cache = status::Theme::new(&args.name);
    cache.save().unwrap();

    log::info!("Current theme: {}", args.name);
}
