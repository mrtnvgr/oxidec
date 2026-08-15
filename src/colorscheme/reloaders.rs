use crate::cache::status::{Colorscheme, Object};
use crate::config::Directory;
use std::process::Command;

pub fn run() {
    if let Some(x) = Colorscheme::try_load() {
        run_reloaders(&x.name);
    } else {
        log::warn!("No colorscheme to reload");
    }
}

fn run_reloaders(colorscheme: &str) {
    for reloader in Directory::Reloaders.list() {
        let mut command = Command::new("sh");

        command.arg("-C").arg(&reloader);
        command.env("OXIDEC_COLORSCHEME", colorscheme);

        let file_name = reloader.file_name().unwrap_or_default();

        match command.status() {
            Ok(status) if !status.success() => {
                log::error!("Error occurred in {file_name:?}");
            }
            Err(error) => log::error!("Failed to run {file_name:?}: {error}"),
            _ => (),
        }
    }
}
