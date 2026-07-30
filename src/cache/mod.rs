pub mod status;
pub mod templates;

use home::home_dir;
use std::fs;

pub fn ensure_cache_exists() {
    let mut path = home_dir().expect("Failed to get HOME directory");
    path.push(".cache/oxidec/templates");
    fs::create_dir_all(path).expect("Failed to create cache directories");
}
