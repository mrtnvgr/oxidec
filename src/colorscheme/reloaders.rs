use crate::config::Folder;
use std::process::Command;

pub fn run() {
    for reloader in Folder::Reloaders.list() {
        let status = Command::new("sh").arg("-C").arg(&reloader).status();

        if !status.unwrap().success() {
            log::error!("Error occured in {:?}", reloader.file_name().unwrap());
        }
    }
}
