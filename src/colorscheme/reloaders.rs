use crate::{cache::status::Colorscheme, config::Folder};
use std::process::Command;

pub fn run() {
    let colorscheme = Colorscheme::load().name;

    for reloader in Folder::Reloaders.list() {
        let mut command = Command::new("sh");

        command.arg("-C").arg(&reloader);
        command.env("OXIDEC_COLORSCHEME", &colorscheme);

        let status = command.status();

        if !status.unwrap().success() {
            log::error!("Error occured in {:?}", reloader.file_name().unwrap());
        }
    }
}
