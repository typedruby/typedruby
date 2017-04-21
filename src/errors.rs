use std::io::Write;

use ast::{Loc, SourceFile};

pub trait ErrorSink {
    fn error(&mut self, message: &str, details: &[(&str, &SourceFile, &Loc)]);
    fn error_count(&self) -> usize;
}

pub struct ErrorReporter<T: Write> {
    io: T,
    need_newline_padding: bool,
    error_count: usize,
}

impl<T: Write> ErrorReporter<T> {
    pub fn new(io: T) -> ErrorReporter<T> {
        ErrorReporter {
            io: io,
            need_newline_padding: false,
            error_count: 0,
        }
    }
}

impl<T: Write> ErrorSink for ErrorReporter<T> {
    fn error(&mut self, message: &str, details: &[(&str, &SourceFile, &Loc)]) {
        self.error_count += 1;

        if self.need_newline_padding {
            write!(self.io, "\n\n").unwrap();
        }

        write!(self.io, "\x1b[31;1merror:\x1b[0;1m {}\x1b[0m\n\n", message).unwrap();

        for &(ref message, ref source, ref loc) in details {
            let begin = source.line_for_pos(loc.begin_pos);
            let end = source.line_for_pos(loc.end_pos);

            if begin.number == end.number {
                // same line
                let line_info = begin;

                write!(self.io, "        \x1b[34;1m@ {}\x1b[0m\n", source.filename()).unwrap();
                write!(self.io, "\x1b[34;1m{number:>7} |\x1b[0m {line_prefix}\x1b[31;1m{highlight}\x1b[0m{line_suffix}\n",
                    number = line_info.number,
                    line_prefix = &source.source()[line_info.begin_pos..loc.begin_pos],
                    highlight = &source.source()[loc.begin_pos..loc.end_pos],
                    line_suffix = &source.source()[loc.end_pos..line_info.end_pos].trim_right()).unwrap();

                write!(self.io, "{pad:pad_len$}\x1b[31;1m{marker}\x1b[0;1m {message}\x1b[0m\n",
                    pad = "", pad_len = 10 + loc.begin_pos - line_info.begin_pos,
                    marker = "^".repeat(loc.end_pos - loc.begin_pos),
                    message = message).unwrap();
            } else {
                panic!("unimplemented: loc spanning multiple lines!")
            }
        }

        self.need_newline_padding = true;
    }

    fn error_count(&self) -> usize {
        self.error_count
    }
}
