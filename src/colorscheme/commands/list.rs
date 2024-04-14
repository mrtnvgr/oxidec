use crate::{colorscheme::args, config::Directory};

pub fn handle(args: &args::List) {
    if args.json {
        output_using_json();
    } else {
        output_using_log();
    }
}

fn output_using_log() {
    log::info!("Colorschemes: ");
    for entry in Directory::Colorschemes.list_stems() {
        log::info!("{:?}", entry);
    }
}

fn output_using_json() {
    let list = Directory::Colorschemes.list_stems();
    print!("{}", serde_json::to_string(&list).unwrap());
}
