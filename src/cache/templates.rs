use home::home_dir;
use std::fs;

pub fn create(name: &str, contents: String) -> std::io::Result<()> {
    let mut path = home_dir().expect("Failed to get HOME directory");
    path.push(".cache/oxidec/templates");
    fs::write(path.join(name), contents)
}
