use crate::{colorscheme::args, config::Folder};

pub fn handle(args: args::List) {
    if args.json {
        output_using_json();
    } else {
        output_using_log();
    }
}

fn output_using_log() {
    log::info!("Colorschemes: ");
    for entry in Folder::Colorschemes.list_stems() {
        log::info!("{:?}", entry);
    }
}

fn output_using_json() {
    let list = Folder::Colorschemes.list_stems();
    print!("{}", serde_json::to_string(&list).unwrap());
}
