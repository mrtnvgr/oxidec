use crate::{colorscheme::args, config::Folder, theme::schema};

pub fn handle(args: &args::Remove) {
    assert!(
        Folder::Colorschemes.contains(&args.name),
        "This colorscheme does not exist"
    );

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
        Ok(()) => log::info!("Colorscheme was deleted successfully"),
        Err(error) => log::error!("Failed to delete a colorscheme: {}", error),
    }
}
