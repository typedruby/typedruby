extern crate difference;
extern crate glob;

use difference::{Changeset, Difference};
use glob::glob;
use std::env;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::{PathBuf};
use std::process::Command;

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

fn typedruby_exe() -> PathBuf {
    bin_dir().join(format!("typedruby{}", env::consts::EXE_SUFFIX))
}

struct Mismatch {
    path: PathBuf,
    expected: String,
    got: String,
}

fn output_path(path: &PathBuf) -> PathBuf {
    let mut expected_file = path.clone();
    expected_file.set_extension("out");
    expected_file
}

fn compare_fixture(path: PathBuf) -> Option<Mismatch> {
    let status = Command::new(typedruby_exe())
        .arg(&path)
        .output()
        .expect("Failed to execute typedruby");

    // TODO: `typedruby` should likely exit 1 if errors are present,
    // and we need to encode exit status into the test somehow.
    assert!(status.status.success(),
            format!("Typechecker exited with status {} on {}", status.status, path.display()));

    let expected_file = output_path(&path);
    let f = File::open(expected_file);
    let mut expected: String = String::new();
    match f {
        Ok(mut file) => file.read_to_string(&mut expected).expect("read failed"),
        Err(..) => 0,
    };

    let rootdir = env::current_dir().unwrap();
    let stderr = String::from_utf8(status.stderr)
        .expect("Output was invalid UTF-8")
        .replace(rootdir.to_str().expect("invalid utf-8 in path"), "__ROOT__");

    if stderr != expected {
        return Some(Mismatch{
            path: path,
            expected: expected,
            got: stderr,
        })
    }
    return None
}

#[test]
fn test_fixtures() {
    let rewrite = match env::var("TYPEDRUBY_UPDATE_FIXTURES") {
        Ok(ref val) => val != "",
        Err(..) => false,
    };

    let mut errors = Vec::new();
    for entry in glob("tests/fixtures/*.rb").expect("glob failed") {
        let path = entry.expect("failed to glob path");
        println!("checking: {}...", path.display());
        if let Some(mismatch) = compare_fixture(path) {
            errors.push(mismatch);
        }
    }
    for err in errors.iter() {
        println!("# {}: output mismatch", err.path.display());
        println!("# diff (-expected, +actual)");

        let ch = Changeset::new(&err.expected, &err.got, "\n");
        for diff in ch.diffs.iter() {
            match *diff {
                Difference::Same(ref s) => println!(" {}", s),
                Difference::Add(ref s) => println!("+{}", s),
                Difference::Rem(ref s) => println!("-{}", s),
            }
        }
        println!("# end diff");

        if rewrite {
            let outpath = output_path(&err.path);
            let mut f = File::create(&outpath)
                .expect(&format!("Unable to open output file: {}", outpath.display()));
            f.write_all(err.got.as_bytes()).expect("error updating output");
        }
    }
    assert!(errors.len() == 0);
}
