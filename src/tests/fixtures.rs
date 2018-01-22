use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

extern crate difference;
use self::difference::{Changeset, Difference};

use glob::glob;
use regex::{Regex, Captures};
use termcolor::NoColor;
use typed_arena::Arena;

use environment::Environment;
use report::TerminalReporter;
use project;

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
    let root_dir = env::current_dir().unwrap();

    let mut error_buff = Vec::new();

    {
        let mut errors = TerminalReporter::new(NoColor::new(&mut error_buff));

        let project = project::load_fixture(&mut errors, &path);

        let arena = Arena::new();
        let env = Environment::new(&arena, &project, &mut errors);

        env.load_files(project.check_config.files.iter());
        env.define();
        env.typecheck();
    }

    let expected = read_file(&output_path(&path));

    let output = String::from_utf8(error_buff).expect("output should be utf-8");
    let output = clean_typecheck_output(&output, &root_dir);

    if output != expected {
        return Some(Mismatch{
            path: path,
            expected: expected,
            got: output,
        })
    }
    return None
}

#[test]
fn test_fixtures() {
    env::set_var("TYPEDRUBY_LIB", env::current_dir().unwrap().join("definitions/lib"));

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
