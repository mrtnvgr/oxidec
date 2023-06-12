use std::{
    env,
    ffi::OsString,
    fs,
    path::{Path, PathBuf},
    process::Command,
};

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=examples");

    #[cfg(windows)]
    panic!("This program is Linux only.");

    if let Some(home_dir) = env::var_os("HOME") {
        copy_example_folder(&home_dir);
    }
}

fn copy_example_folder(home_dir: &OsString) {
    let config_path = Path::new(&home_dir).join(".config/oxidec/");
    let repo_path = Path::new("examples");

    for folder in ["templates", "reloaders"] {
        let user_folder = config_path.join(folder);
        let repo_folder = repo_path.join(folder);

        for repo_file in fs::read_dir(repo_folder).unwrap().flatten() {
            let file_name = repo_file.file_name();
            let user_file = user_folder.join(file_name);

            if !user_file.exists() {
                copy_file(repo_file.path(), user_file);
            }
        }
    }
}

fn copy_file(src: PathBuf, dest: PathBuf) {
    Command::new("cp")
        .arg("-n")
        .arg(src)
        .arg(dest)
        .output()
        .unwrap();
}
