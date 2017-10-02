use std::env;
use std::path::PathBuf;

// Path to our executables
fn bin_dir() -> PathBuf {
    env::current_exe().ok().map(|mut path| {
        path.pop();
        if path.ends_with("deps") {
            path.pop();
        }
        path
    }).unwrap_or_else(|| {
        panic!("Can't find bin directory.")
    })
}

pub fn typedruby_exe() -> PathBuf {
    bin_dir().join(format!("typedruby{}", env::consts::EXE_SUFFIX))
}