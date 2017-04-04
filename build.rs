use std::env;
use std::process::Command;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    Command::new("make").status().unwrap();

    println!("cargo:rustc-link-search=native={}", env::current_dir().unwrap().display());
}
