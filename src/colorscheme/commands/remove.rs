use crate::{colorscheme::args, config::Folder, theme::schema};
use std::io::ErrorKind;

pub fn handle(args: &args::Remove) {
    for path in Folder::Themes.list() {
        let theme = schema::Theme::from_file(&path).unwrap();
        let path = Folder::Colorschemes.get(&args.name).unwrap();

        assert!(
            theme.colorscheme.path != path,
            "\"{}\" theme depends on this colorscheme",
            theme.name
        );
    }

    match Folder::Colorschemes.remove(&args.name) {
        Ok(_) => log::info!("Colorscheme was deleted successfully"),
        Err(error) if error.kind() == ErrorKind::NotFound => {
            log::error!("This colorscheme does not exist");
        }
        Err(error) => log::error!("Failed to delete a colorscheme: {}", error),
    }
}
