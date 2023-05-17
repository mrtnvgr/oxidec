use crate::{colorscheme::args, config::Folder};

pub fn handle(args: &args::List) {
    if args.json {
        output_using_json();
    } else {
        output_using_log();
    }
}

fn output_using_log() {
    log::info!("Colorschemes: ");
    for entry in Folder::Colorschemes.list().unwrap() {
        let colorscheme = entry.file_stem().unwrap();
        log::info!("- {:?}", colorscheme);
    }
}

fn output_using_json() {
    let list = Folder::Colorschemes.list().unwrap();
    print!("{}", serde_json::to_string(&list).unwrap());
}
