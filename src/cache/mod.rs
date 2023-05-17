pub mod status;
pub mod templates;

use home::home_dir;
use std::fs;

pub fn ensure_cache_exists() {
    let mut home = home_dir().expect("Failed to get HOME directory");
    home.push(".cache/oxidec/");
    for path in [home.join("templates"), home.join("status")] {
        fs::create_dir_all(path).expect("Failed to create config directories");
    }
}
