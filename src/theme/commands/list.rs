use crate::{config::Folder, theme::args};

pub fn handle(args: &args::List) {
    let themes = Folder::Themes.list_stems();

    if args.json {
        output_using_json(&themes);
    } else {
        output_using_log(&themes);
    }
}

fn output_using_log(themes: &[String]) {
    log::info!("Themes: ");
    for entry in themes {
        log::info!("{:?}", entry);
    }
}

fn output_using_json(themes: &[String]) {
    print!("{}", serde_json::to_string(&themes).unwrap());
}
