use crate::{config::Directory, state::args};

pub fn handle(args: &args::List) {
    let states = Directory::States.list_stems();

    if args.json {
        output_using_json(&states);
    } else {
        output_using_log(&states);
    }
}

fn output_using_log(states: &[String]) {
    log::info!("Saved states: ");
    for entry in states {
        log::info!("{:?}", entry);
    }
}

fn output_using_json(states: &[String]) {
    print!("{}", serde_json::to_string(&states).unwrap());
}
