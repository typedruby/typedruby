use termcolor::{Color, ColorSpec, WriteColor};
use std::io::{Result};
use ast::{Loc, Diagnostic, Level};

pub enum Detail<'a> {
    Message(&'a str),
    Loc(&'a str, &'a Loc),
}

pub trait ErrorSink {
    fn error(&mut self, message: &str, details: &[Detail]);
    fn warning(&mut self, message: &str, details: &[Detail]);

    fn error_count(&self) -> usize;
    fn warning_count(&self) -> usize;

    fn parser_diagnostic(&mut self, diagnostic: &Diagnostic) {
        match diagnostic.level {
            Level::Note => {}
            Level::Warning => {}
            Level::Error | Level::Fatal => {
                self.error(&format!("{}", diagnostic), &[
                    Detail::Loc("here", &diagnostic.loc),
                ]);
            }
        }
    }
}

fn color_scheme(base: Color) -> (ColorSpec, ColorSpec, ColorSpec) {
    let mut main = ColorSpec::new();
    main.set_fg(Some(base));
    main.set_bold(true);

    let mut high = ColorSpec::new();
    high.set_bold(true);

    let mut low = ColorSpec::new();
    low.set_fg(Some(Color::Blue));
    low.set_intense(false);
    low.set_bold(true);

    (main, high, low)
}

macro_rules! write_color {
    ($color:expr, $io:expr, $($arg:tt)*) => ({
        $io.set_color(&$color)?;
        write!($io, $($arg)*)?;
        $io.reset()?;
    });
}

pub struct ErrorReporter<T: WriteColor> {
    io: T,
    need_newline_padding: bool,
    error_count: usize,
    warning_count: usize,
}

impl<T: WriteColor> ErrorReporter<T> {
    pub fn new(io: T) -> ErrorReporter<T> {
        ErrorReporter {
            io: io,
            need_newline_padding: false,
            error_count: 0,
            warning_count: 0,
        }
    }

    fn emit(&mut self, diagnostic_name: &str, color: Color, message: &str, details: &[Detail]) -> Result<()> {
        let (err, high, low) = color_scheme(color);

        if self.need_newline_padding {
            write!(self.io, "\n")?;
        }

        write_color!(err, self.io, "{}: ", diagnostic_name);
        write_color!(high, self.io, "{}\n\n", message);

        self.io.reset()?;

        for detail in details {
            match *detail {
                Detail::Loc(ref message, ref loc) => {
                    let begin = loc.file().line_for_pos(loc.begin_pos);
                    let end = loc.file().line_for_pos(loc.end_pos);

                    write_color!(low, self.io, "        @ {}:{}\n",
                        loc.file().filename().display(),
                        begin.number);

                    if begin.number == end.number {
                        // same line
                        let line_info = begin;
                        let source = loc.file().source();

                        write_color!(low, self.io, "{:>7} | ", line_info.number);

                        write!(self.io, " {}", &source[line_info.begin_pos..loc.begin_pos])?;

                        write_color!(err, self.io, "{}", &source[loc.begin_pos..loc.end_pos]);

                        write!(self.io, "{}\n", source[loc.end_pos..line_info.end_pos].trim_right())?;

                        write_color!(err, self.io, "{0:1$}{2}",
                           "", 11 + loc.begin_pos - line_info.begin_pos,
                           "^".repeat(loc.end_pos - loc.begin_pos)
                        );

                        write_color!(high, self.io, " {}\n", message);
                    } else {
                        let source = loc.file().source()[begin.begin_pos..end.end_pos].split("\n");

                        write_color!(err, self.io, "{0:1$}{2}v\n",
                            "", 10, "-".repeat(loc.begin_pos - begin.begin_pos + 1)
                        );

                        for (line_no, line) in (begin.number..(end.number + 1)).zip(source) {
                            write_color!(low, self.io, "{:>7} | ", line_no);
                            write_color!(err, self.io, "|");
                            write!(self.io, "{}\n", line.trim_right())?;
                        }

                        write_color!(err, self.io, "{0:1$}{2}^",
                            "", 10, "-".repeat(loc.end_pos - end.begin_pos)
                        );

                        write_color!(high, self.io, " {}\n", message);
                    }
                },
                Detail::Message(ref message) => {
                    write_color!(low, self.io, "\n        - ");
                    write_color!(high, self.io, "{}\n\n", message);
                }
            }
        }

        self.need_newline_padding = details.len() > 0;
        Ok(())
    }
}

impl<T: WriteColor> ErrorSink for ErrorReporter<T> {
    fn error(&mut self, message: &str, details: &[Detail]) {
        self.error_count += 1;
        self.emit("error", Color::Red, message, details).unwrap();
    }

    fn warning(&mut self, message: &str, details: &[Detail]) {
        self.error_count += 1;
        self.emit("warning", Color::Yellow, message, details).unwrap();
    }

    fn error_count(&self) -> usize {
        self.error_count
    }

    fn warning_count(&self) -> usize {
        self.warning_count
    }
}
