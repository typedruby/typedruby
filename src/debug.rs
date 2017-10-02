use std::rc::Rc;
use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use ast::SourceFile;
use strip::ByteRange;

pub fn annotate_file(file: &Rc<SourceFile>, ranges: &[ByteRange]) {
    let mut stderr = StandardStream::stderr(ColorChoice::Always);

    let mut red = ColorSpec::new();
    red.set_fg(Some(Color::Red));

    let mut dark = ColorSpec::new();
    dark.set_fg(Some(Color::Green));

    stderr.set_color(&dark).unwrap();
    writeln!(&mut stderr, "###### src: {}", file.filename().display()).unwrap();
    stderr.reset().unwrap();

    let source = file.source();
    let mut offset = 0;
    let mut lineno = 1;
    for line in source.split("\n") {
        let end = offset + line.len();

        stderr.set_color(&dark).unwrap();
        write!(&mut stderr, "{:4} | ", lineno).unwrap();
        stderr.reset().unwrap();
        writeln!(&mut stderr, "{}", line).unwrap();

        for range in ranges.iter() {
            let &ByteRange(range_start, range_end) = range;
            if range_start >= offset && range_end <= end {
                let pos = range_start - offset;
                let len = range_end - range_start;

                stderr.set_color(&red).unwrap();
                writeln!(&mut stderr, "       {0:1$}{2}", "", pos, "^".repeat(len)).unwrap();
                stderr.reset().unwrap();
            }
        }

        offset = end + 1;
        lineno = lineno + 1;
    }

    writeln!(&mut stderr, "").unwrap();
}
