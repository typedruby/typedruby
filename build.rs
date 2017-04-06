use std::env;
use std::process;
use std::process::Command;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    let mut cmd = Command::new("make");

    cmd.env("LIB_PATH", out_dir.clone() + "/librubyparser.a");

    if !cmd.status().unwrap().success() {
        process::exit(1);
    }

    println!("cargo:rustc-link-search=native={}", out_dir);
}
