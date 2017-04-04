use std::env;
use std::process;
use std::process::Command;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    if !Command::new("make").status().unwrap().success() {
        process::exit(1);
    }

    println!("cargo:rustc-link-search=native={}", env::current_dir().unwrap().display());
}
