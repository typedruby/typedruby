use termcolor::{Color, ColorSpec, WriteColor};
use std::io::{Result};
use ast::Loc;

pub enum Detail<'a> {
    Message(&'a str),
    Loc(&'a str, &'a Loc),
}

pub trait ErrorSink {
    fn error(&mut self, message: &str, details: &[Detail]);
    fn warning(&mut self, message: &str, details: &[Detail]);

    fn error_count(&self) -> usize;
    fn warning_count(&self) -> usize;
}

fn color_scheme(base: Color) -> (ColorSpec, ColorSpec, ColorSpec) {
    let mut main = ColorSpec::new();
    main.set_fg(Some(base));
    main.set_bold(true);

    let mut high = ColorSpec::new();
    high.set_bold(true);

    let mut low = ColorSpec::new();
    low.set_fg(Some(Color::Cyan));
    low.set_intense(false);
    low.set_bold(true);

    (main, high, low)
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
        let (color_err, color_high, color_low) = color_scheme(color);

        if self.need_newline_padding {
            write!(self.io, "\n")?;
        }

        self.io.set_color(&color_err)?;
        write!(self.io, "{}: ", diagnostic_name)?;

        self.io.set_color(&color_high)?;
        write!(self.io, "{}\n\n", message)?;

        self.io.reset()?;

        for detail in details {
            match *detail {
                Detail::Loc(ref message, ref loc) => {
                    let begin = loc.file.line_for_pos(loc.begin_pos);
                    let end = loc.file.line_for_pos(loc.end_pos);

                    self.io.set_color(&color_low)?;
                    write!(self.io, "        @ {}:{}\n",
                           loc.file.filename().display(),
                           begin.number)?;
                    self.io.reset()?;

                    if begin.number == end.number {
                        // same line
                        let line_info = begin;

                        self.io.set_color(&color_low)?;
                        write!(self.io, "{:>7} | ", line_info.number)?;

                        self.io.reset()?;
                        write!(self.io, " {}", 
                               &loc.file.source()[line_info.begin_pos..loc.begin_pos])?;

                        self.io.set_color(&color_err)?;
                        write!(self.io, "{}", 
                               &loc.file.source()[loc.begin_pos..loc.end_pos])?;

                        self.io.reset()?;
                        write!(self.io, "{}\n",
                               &loc.file.source()[loc.end_pos..line_info.end_pos].trim_right())?;

                        self.io.set_color(&color_err)?;
                        write!(self.io, "{pad:pad_len$}{marker}",
                               pad = "", pad_len = 11 + loc.begin_pos - line_info.begin_pos,
                               marker = "^".repeat(loc.end_pos - loc.begin_pos))?;

                        self.io.set_color(&color_high)?;
                        write!(self.io, " {}\n", message)?;
                    } else {
                        let source = loc.file.source()[begin.begin_pos..end.end_pos].split("\n");

                        self.io.set_color(&color_err)?;
                        write!(self.io, "{pad:pad_len$}{marker}v\n",
                            pad = "", pad_len = 10,
                            marker = "-".repeat(loc.begin_pos - begin.begin_pos + 1)
                        )?;
                        self.io.reset()?;

                        for (line_no, line) in (begin.number..(end.number + 1)).zip(source) {
                            self.io.set_color(&color_low)?;
                            write!(self.io, "{number:>7} | ", number = line_no)?;

                            self.io.set_color(&color_err)?;
                            write!(self.io, "|")?;

                            self.io.reset()?;
                            write!(self.io, "{}\n", line.trim_right())?;
                        }

                        self.io.set_color(&color_err)?;
                        write!(self.io, "{pad:pad_len$}{marker}^",
                            pad = "", pad_len = 10,
                            marker = "-".repeat(loc.end_pos - end.begin_pos))?;

                        self.io.set_color(&color_high)?;
                        write!(self.io, " {}\n", message)?;
                    }
                },
                Detail::Message(ref message) => {
                    self.io.set_color(&color_low)?;
                    write!(self.io, "\n        - ")?;
                    
                    self.io.set_color(&color_high)?;
                    write!(self.io, "{}\n\n", message)?;
                }
            }

            self.io.reset()?;
        }

        self.need_newline_padding = true;
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
