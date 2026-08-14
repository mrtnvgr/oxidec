use crate::cache::CACHE_DIR;
use std::{fs, io};

pub fn create(name: &str, contents: String) -> io::Result<()> {
    fs::write(CACHE_DIR.join("templates").join(name), contents)
}
