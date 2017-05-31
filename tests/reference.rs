extern crate difference;
extern crate glob;
extern crate ruby_parser;

use glob::glob;
use std::rc::Rc;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use difference::{Changeset, Difference};

fn known_failure(path: &PathBuf) -> bool {
    match path.to_str().unwrap() {
        // Unicode escaping issues
        "library/04/366f4149a4e8cbed776ef942ac9c4b.rb" => true,
        "library/1d/5e4e1adaa1ba949947a08eeff4111a.rb" => true,
        "library/24/510f9519e650e76d8849acf95bf073.rb" => true,
        "library/30/5d03654f76d5286175a6d78101ea13.rb" => true,
        "library/31/6e1e3598a45c719b623e3878d3e98f.rb" => true,
        "library/8b/69110e8901f40f6119a66d116075a2.rb" => true,
        "library/a1/cd831b43b2be9f67eb8c6eb008cba1.rb" => true,
        "library/b2/650be44867235f5f9ac99ea9f3554e.rb" => true,
        "library/c8/08230a8cd26f0d686f4784ad3c49a0.rb" => true,

        // Mlhs
        "library/32/ae1517d8058d13e7948d890377bf77.rb" => true,
        "library/bf/48d40cd135bd8083711e089549b373.rb" => true,

        // match-with-lvasgn
        "library/a4/706545d5c4055aec5b11c904aeaa79.rb" => true,

        // Anything else should be green
        _ => false,
    }
}

fn compare_sexps(path: &PathBuf) {
    let mut buf_rs = String::new();
    let mut buf_rb = String::new();

    let sexp_path = path.with_extension("sexp");
    let src = ruby_parser::SourceFile::open(&path).expect("failed to load file");
    let ast = ruby_parser::parse(Rc::new(src));

    {
        let mut file = File::open(&sexp_path).unwrap();
        file.read_to_string(&mut buf_rb).expect("failed to read sexp");
        assert!(buf_rb.len() > 0, "empty file at {}", sexp_path.display());
    }

    ast.to_sexp(&mut buf_rs).expect("failed to write sexp output");
    assert!(buf_rs.len() > 0);

    if buf_rs.len() != buf_rb.len() {
        let ch = Changeset::new(buf_rs.as_str(), buf_rb.as_str(), "\n");
        if ch.distance != 0 {
            println!("Mismatch in '{}':", path.display());
            for d in &ch.diffs {
                match *d {
                    Difference::Add(ref x) => {
                        println!("\x1b[92m{}\x1b[0m", x);
                    }
                    Difference::Rem(ref x) => {
                        println!("\x1b[91m{}\x1b[0m", x);
                    }
                    Difference::Same(_) => {}
                }
            }
        }
        assert_eq!(ch.distance, 0);
    }
}

#[test]
#[ignore]
fn compare_full_library() {
    for entry in glob("library/**/*.rb").unwrap() {
        let path = entry.expect("failed to glob path");
        if !known_failure(&path) {
            compare_sexps(&path);
        }
    }
}

#[test]
fn compare_sinatra() {
    let path = PathBuf::from("tests/fixtures/sinatra.rb");
    compare_sexps(&path);
}
