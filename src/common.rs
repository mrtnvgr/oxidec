use crate::config::Directory;
use serde::Serialize;

pub fn handle_list(names: Vec<String>, json: bool) {
    if json {
        print_json(&names);
    } else {
        for name in names {
            log::info!("{name}");
        }
    }
}

pub fn print_json<T: Serialize>(value: &T) {
    print!("{}", serde_json::to_string(value).unwrap());
}

pub fn handle_remove(directory: Directory, name: &str, what: &str) {
    assert!(directory.contains(name), "This {what} does not exist");

    match directory.remove(name) {
        Ok(()) => log::info!("\"{name}\" {what} was deleted successfully"),
        Err(error) => log::error!("Failed to delete a {what}: {error}"),
    }
}
