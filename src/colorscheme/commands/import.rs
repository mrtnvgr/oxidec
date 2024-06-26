use crate::{
    colorscheme::{args, schema},
    config::Directory,
};

pub fn handle(args: &args::Import) {
    assert!(args.file_path.exists(), "This file does not exist");

    let filename = args.file_path.file_stem().unwrap();
    let name = filename.to_str().unwrap();
    assert!(
        !Directory::Colorschemes.contains(name),
        "Colorscheme with this name already exists!"
    );

    let extension = args.file_path.extension().unwrap_or_default();
    match extension.to_ascii_lowercase().to_str().unwrap_or_default() {
        "json" => handle_json(args),
        // TODO: "xres" | "xresources" => todo!(),
        _ => panic!("This file type is not supported."),
    }

    log::info!("Imported successfully as \"{}\"!", name);
}

fn handle_json(args: &args::Import) {
    schema::Colorscheme::from_file(&args.file_path).expect("Failed to validate this JSON file");
    Directory::Colorschemes.copy(&args.file_path).unwrap();
}
