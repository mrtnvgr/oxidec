fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    #[cfg(windows)]
    panic!("This program is not suitable for Windows systems.");
}
