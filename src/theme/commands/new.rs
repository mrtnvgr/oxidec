use crate::{
    cache::status::{self, Object},
    config::Folder,
    theme::{args, schema},
};

use std::fs::File;

pub fn handle(args: &args::New) {
    assert!(
        !Folder::Themes.contains(&args.name),
        "A theme with this name already exists"
    );

    let colorscheme = status::Colorscheme::load();
    let wallpaper = status::Wallpaper::load();

    let theme = schema::Theme::new(&args.name, colorscheme, vec![wallpaper]);

    let path = Folder::Themes.build_path(&args.name);
    serde_json::to_writer(File::create(path).unwrap(), &theme).unwrap();

    log::info!("{} theme was saved", args.name);
}
