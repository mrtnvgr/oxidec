use std::{env, fs, path::Path, process::Command};

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=examples");

    #[cfg(windows)]
    panic!("This program is Linux only.");

    if let Some(home_dir) = env::var_os("HOME") {
        let config_path = Path::new(&home_dir).join(".config/oxidec/");

        if cfg!(feature = "new-examples") {
            copy_new_examples(&config_path);
        } else {
            copy_examples_folder(&config_path);
        }
    }
}

fn copy_examples_folder(config_path: &Path) {
    if !config_path.exists() {
        copy_folder("examples", config_path);
    }
}

fn copy_new_examples(config_path: &Path) {
    let repo_path = Path::new("examples");

    for folder in ["templates", "reloaders"] {
        let user_folder = config_path.join(folder);
        let repo_folder = repo_path.join(folder);

        for repo_file in fs::read_dir(repo_folder).unwrap().flatten() {
            let file_name = repo_file.file_name();
            let user_file = user_folder.join(file_name);

            if !user_file.exists() {
                fs::copy(repo_file.path(), user_file).unwrap();
            }
        }
    }
}

fn copy_folder(src: &str, dest: &Path) {
    let mut command = Command::new("cp");
    command.arg("-r");
    command.arg(src);
    command.arg(dest);

    command.output().unwrap();
}
