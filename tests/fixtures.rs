extern crate difference;
extern crate glob;
extern crate regex;
#[macro_use]
extern crate lazy_static;

mod common;

use difference::{Changeset, Difference};
use glob::glob;
use std::env;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;
use regex::{Regex, Captures};


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

fn read_file(path: &Path) -> String {
    let mut f = File::open(path).expect("open to succeed");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("read to succeed");
    contents
}

lazy_static! {
    static ref STDLIB_LINE_NUMBER_RE: Regex = Regex::new(r"(?m)^ {8}@ \(builtin stdlib\):(\d+)\n\s+(\d+) \|").unwrap();
    static ref NUMBER_RE: Regex = Regex::new(r"\d+").unwrap();
}

fn clean_stdlib_line_numbers(output: &str) -> String {
    STDLIB_LINE_NUMBER_RE.replace_all(output, |caps: &Captures|
        NUMBER_RE.replace_all(&caps[0], "###").into_owned()).into_owned()
}

fn clean_typecheck_output(output: &str, rootdir: &Path) -> String {
    let output = output.replace(
        rootdir.to_str().expect("rootdir to be a valid utf-8 path"), "__ROOT__");

    clean_stdlib_line_numbers(&output)
}

fn compare_fixture(path: PathBuf) -> Option<Mismatch> {
    let rootdir = env::current_dir().unwrap();

    let status = Command::new(common::typedruby_exe())
        .arg("check")
        .arg("-I")
        .arg(rootdir.join("definitions/lib"))
        .arg(&path)
        // Remove TERM to force termcolor to not output colors in
        // tests.
        .env_remove("TERM")
        .output()
        .expect("Failed to execute typedruby");

    let expected = read_file(&output_path(&path));
    let expected_code = match expected.len() {
        0 => 0,
        _ => 1,
    };

    let exit_code = status.status.code().expect("process to exit cleanly with a status code");

    assert_eq!(expected_code, exit_code,
        "unexpected exit code when typechecking '{}' (want: {}, got: {})",
        path.display(), expected_code, exit_code);

    let stderr = String::from_utf8(status.stderr)
        .expect("output to be utf-8");

    let stderr = clean_typecheck_output(&stderr, &rootdir);

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
            let (diffchar, s) = match *diff {
                Difference::Same(ref s) => (' ', s),
                Difference::Add(ref s) => ('+', s),
                Difference::Rem(ref s) => ('-', s),
            };
            for line in s.split("\n") {
                println!("{}{}", diffchar, line);
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
