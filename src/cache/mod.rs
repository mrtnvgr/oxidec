pub mod status;
pub mod templates;

use home::home_dir;
use std::fs;
use std::path::PathBuf;
use std::sync::LazyLock;

pub static CACHE_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
    let mut path = home_dir().expect("Failed to get HOME directory");
    path.push(".cache/oxidec");
    path
});

pub fn ensure_cache_exists() {
    fs::create_dir_all(CACHE_DIR.join("templates")).expect("Failed to create cache directories");
}
