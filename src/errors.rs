use std::io::Write;

use ast::Loc;

pub trait ErrorSink {
    fn error(&mut self, details: &[(String, Loc)]);
    fn error_count(&self) -> usize;
}

pub struct ErrorReporter<T: Write> {
    io: T,
    error_count: usize,
}

impl<T: Write> ErrorReporter<T> {
    pub fn new(io: T) -> ErrorReporter<T> {
        ErrorReporter {
            io: io,
            error_count: 0,
        }
    }
}

impl<T: Write> ErrorSink for ErrorReporter<T> {
    fn error(&mut self, details: &[(String, Loc)]) {
        self.error_count += 1;

        assert!(details.len() >= 1);
        let (ref main_message, ref main_location) = *details.first().unwrap();
        write!(self.io, "\x1b[31;1merror:\x1b[0;1m {}\x1b[0m\n", main_message);

        // TODO print location
    }

    fn error_count(&self) -> usize {
        self.error_count
    }
}
