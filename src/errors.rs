use std::io::Write;

use ast::{Loc, SourceFile};

type Details<'a> = [(&'a str, &'a SourceFile, &'a Loc)];

pub trait ErrorSink {
    fn error(&mut self, message: &str, details: &Details);
    fn warning(&mut self, message: &str, details: &Details);

    fn error_count(&self) -> usize;
    fn warning_count(&self) -> usize;
}

pub struct ErrorReporter<T: Write> {
    io: T,
    need_newline_padding: bool,
    error_count: usize,
    warning_count: usize,
}

impl<T: Write> ErrorReporter<T> {
    pub fn new(io: T) -> ErrorReporter<T> {
        ErrorReporter {
            io: io,
            need_newline_padding: false,
            error_count: 0,
            warning_count: 0,
        }
    }

    fn emit(&mut self, diagnostic_name: &str, diagnostic_color: usize, message: &str, details: &Details) {
        if self.need_newline_padding {
            write!(self.io, "\n").unwrap();
        }

        write!(self.io, "\x1b[3{diagnostic_color};1m{diagnostic_name}:\x1b[0;1m {message}\x1b[0m\n\n",
            diagnostic_name = diagnostic_name,
            diagnostic_color = diagnostic_color,
            message = message).unwrap();

        for &(ref message, ref source, ref loc) in details {
            let begin = source.line_for_pos(loc.begin_pos);
            let end = source.line_for_pos(loc.end_pos);

            if begin.number == end.number {
                // same line
                let line_info = begin;

                write!(self.io, "        \x1b[34;1m@ {}\x1b[0m\n", source.filename()).unwrap();

                write!(self.io, "\x1b[34;1m{number:>7} |\x1b[0m {line_prefix}\x1b[3{diagnostic_color};1m{highlight}\x1b[0m{line_suffix}\n",
                    diagnostic_color = diagnostic_color,
                    number = line_info.number,
                    line_prefix = &source.source()[line_info.begin_pos..loc.begin_pos],
                    highlight = &source.source()[loc.begin_pos..loc.end_pos],
                    line_suffix = &source.source()[loc.end_pos..line_info.end_pos].trim_right()).unwrap();

                write!(self.io, "{pad:pad_len$}\x1b[3{diagnostic_color};1m{marker}\x1b[0;1m {message}\x1b[0m\n",
                    diagnostic_color = diagnostic_color,
                    pad = "", pad_len = 10 + loc.begin_pos - line_info.begin_pos,
                    marker = "^".repeat(loc.end_pos - loc.begin_pos),
                    message = message).unwrap();
            } else {
                panic!("unimplemented: loc spanning multiple lines!")
            }
        }

        self.need_newline_padding = true;
    }
}

impl<T: Write> ErrorSink for ErrorReporter<T> {
    fn error(&mut self, message: &str, details: &Details) {
        self.error_count += 1;
        self.emit("error", 1, message, details);
    }

    fn warning(&mut self, message: &str, details: &Details) {
        self.error_count += 1;
        self.emit("warning", 3, message, details);
    }

    fn error_count(&self) -> usize {
        self.error_count
    }

    fn warning_count(&self) -> usize {
        self.warning_count
    }
}