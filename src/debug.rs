use std::io;
use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use ast::SourceFile;
use strip::ByteRange;

pub fn annotate_file(file: &SourceFile, ranges: &[ByteRange]) -> io::Result<()> {
    let mut stderr = StandardStream::stderr(ColorChoice::Always);

    let mut red = ColorSpec::new();
    red.set_fg(Some(Color::Red));

    let mut dark = ColorSpec::new();
    dark.set_fg(Some(Color::Green));

    stderr.set_color(&dark)?;
    writeln!(&mut stderr, "###### src: {}", file.filename().display())?;
    stderr.reset()?;

    let source = file.source();
    let mut offset = 0;
    let mut lineno = 1;
    for line in source.split("\n") {
        let end = offset + line.len();

        stderr.set_color(&dark)?;
        write!(&mut stderr, "{:4} | ", lineno)?;
        stderr.reset()?;
        writeln!(&mut stderr, "{}", line)?;

        for &ByteRange(range_start, range_end) in ranges.iter() {
            if range_start >= offset && range_end <= end {
                let pos = range_start - offset;
                let len = range_end - range_start;

                stderr.set_color(&red)?;
                writeln!(&mut stderr, "       {0:1$}{2}", "", pos, "^".repeat(len))?;
                stderr.reset()?;
            }
        }

        offset = end + 1;
        lineno = lineno + 1;
    }

    writeln!(&mut stderr, "")
}
