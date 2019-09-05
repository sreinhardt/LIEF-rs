use std::env;
use std::path::PathBuf;

const LIBRARY_PATH: &str = "LIEF_LIB_PATH";
const INCLUDE_PATH: &str = "LIEF_INC_PATH";
const LIEF_H:       &str = "LIEF.h";
const LIEF_RS:      &str = "lief.rs";

fn main() {
    let inc_path = env::var(INCLUDE_PATH)
        .expect("Set LIEF.h path (lief/api/c...)");
    let out_path = env::var("OUT_DIR")
        .expect("Set OUT_DIR path");
    let paths = vec![
        env::var("CARGO_MANIFEST_DIR"),
        env::var(LIBRARY_PATH)
    ];

    paths.iter()
        .filter(|p| p.is_ok())
        .map(|p| p.as_ref().unwrap())
        .for_each(|p| println!("cargo:rustc-link-search={}/", p));

    if cfg!(r#static) {
        println!("cargo:rustc-link-lib=static=LIEF");
    } else {
        println!("cargo:rustc-link-lib=LIEF");
    }

    let inc_file = format!("{}/{}", inc_path, LIEF_H);
    let inc_flag = format!("-I{}", inc_path);
    let out_file = format!("{}/{}", out_path, LIEF_RS);

    bindgen::Builder::default()
        .header(inc_file)
        //.clang_arg(inc_flag)
        //.clang_arg("-I/mnt/d/Code/github/LIEF/build-nix/include/")
        //.clang_arg("-I/mnt/d/Code/github/LIEF/include/")
        .clang_arg("-I/mnt/d/Code/github/LIEF/build-nix/install/usr/include/")
        .generate()
        .expect("Unable to generate <LIEF.h> bindings")
        .write_to_file(out_file)
        .expect("Couldn't write <LIEF.h> bindings");
}
