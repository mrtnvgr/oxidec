use crate::{
    cache::status::{Colorscheme, Object},
    config::Folder,
};
use std::{env, process::Command};

pub fn run(gtk: bool) {
    let colorscheme = Colorscheme::load().name;

    for reloader in Folder::Reloaders.list() {
        let mut command = Command::new("sh");

        command.arg("-C").arg(&reloader);
        command.env("OXIDEC_COLORSCHEME", &colorscheme);

        if gtk && env::var_os("OXIDEC_GTK").is_none() {
            command.env("OXIDEC_GTK", "y");
        }

        let status = command.status();

        if !status.unwrap().success() {
            log::error!("Error occured in {:?}", reloader.file_name().unwrap());
        }
    }
}
