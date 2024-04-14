use crate::{config::Directory, theme::args};

pub fn handle(args: &args::List) {
    let themes = Directory::Themes.list_stems();

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
