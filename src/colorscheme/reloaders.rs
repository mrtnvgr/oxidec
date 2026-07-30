use crate::cache::status::{Colorscheme, Object};
use crate::config::Directory;
use std::process::Command;

pub fn run() {
    if let Some(x) = Colorscheme::try_load() {
        run_reloaders(&x.name);
    } else {
        log::warn!("No colorscheme is selected");
    }
}

fn run_reloaders(colorscheme: &str) {
    for reloader in Directory::Reloaders.list() {
        let mut command = Command::new("sh");

        command.arg("-C").arg(&reloader);
        command.env("OXIDEC_COLORSCHEME", colorscheme);

        let status = command.status();

        if !status.unwrap().success() {
            log::error!("Error occured in {:?}", reloader.file_name().unwrap());
        }
    }
}
