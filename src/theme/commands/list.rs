use crate::{config::Folder, theme::args};

pub fn handle(args: &args::List) {
    if args.json {
        output_using_json();
    } else {
        output_using_log();
    }
}

fn output_using_log() {
    log::info!("Themes: ");
    for entry in Folder::Themes.list() {
        let theme = entry.file_stem().unwrap();
        log::info!("{:?}", theme);
    }
}

fn output_using_json() {
    let list = Folder::Themes.list();
    print!("{}", serde_json::to_string(&list).unwrap());
}
