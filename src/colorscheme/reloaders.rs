use crate::config::Directory;
use crate::cache::status::{Colorscheme, Object};
use std::process::Command;

pub fn run() {
    let colorscheme = Colorscheme::load().name;

    for reloader in Directory::Reloaders.list() {
        let mut command = Command::new("sh");

        command.arg("-C").arg(&reloader);
        command.env("OXIDEC_COLORSCHEME", &colorscheme);

        let status = command.status();

        if !status.unwrap().success() {
            log::error!("Error occured in {:?}", reloader.file_name().unwrap());
        }
    }
}
