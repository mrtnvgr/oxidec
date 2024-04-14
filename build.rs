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
            copy_examples_directory(&config_path);
        }
    }
}

fn copy_examples_directory(config_path: &Path) {
    if !config_path.exists() {
        copy_directory("examples", config_path);
    }
}

fn copy_new_examples(config_path: &Path) {
    let repo_path = Path::new("examples");

    for directory in ["templates", "reloaders"] {
        let user_directory = config_path.join(directory);
        let repo_directory = repo_path.join(directory);

        for repo_file in fs::read_dir(repo_directory).unwrap().flatten() {
            let file_name = repo_file.file_name();
            let user_file = user_directory.join(file_name);

            if !user_file.exists() {
                fs::copy(repo_file.path(), user_file).unwrap();
            }
        }
    }
}

fn copy_directory(src: &str, dest: &Path) {
    let mut command = Command::new("cp");
    command.arg("-r");
    command.arg(src);
    command.arg(dest);

    command.output().unwrap();
}
