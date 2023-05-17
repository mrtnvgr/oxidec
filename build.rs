use std::{
    env,
    ffi::OsString,
    path::{Path, PathBuf},
    process::Command,
};

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    #[cfg(windows)]
    panic!("This program is Linux only.");

    if let Some(home_dir) = env::var_os("HOME") {
        copy_example_folder(&home_dir);
    }
}

fn copy_example_folder(home_dir: &OsString) {
    let config_path = Path::new(&home_dir).join(".config/oxidec/");
    if !config_path.exists() {
        copy_folder("examples", config_path);
    }
}

fn copy_folder(src: &str, dest: PathBuf) {
    Command::new("cp")
        .arg("-r")
        .arg(src)
        .arg(dest)
        .output()
        .unwrap();
}
