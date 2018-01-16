use std::env;
use std::io::{self, Read, Write};
use std::os::unix::net::UnixListener;
use std::path::PathBuf;
use std::sync::Mutex;

use environment::Environment;
use errors::{self, ErrorSink};
use remote;
use remote::protocol::{self, ProtocolError, Message, ClientTransport, ReplyData, ClientTransaction};

use crossbeam;
use typed_arena::Arena;

#[derive(Debug)]
pub enum RunServerError {
    Io(io::Error)
}

pub fn run() -> Result<(), RunServerError> {
    let path = remote::socket_path().map_err(RunServerError::Io)?;

    let listener = UnixListener::bind(&path).map_err(RunServerError::Io)?;

    let mutex = Mutex::new(());

    crossbeam::scope(|scope| {
        let mut result = Ok(());

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    let mutex = &mutex;
                    scope.spawn(move || {
                        match Client::run(stream, mutex) {
                            Ok(()) => {}
                            Err(e) => {
                                eprintln!("error in client thread: {:?}", e);
                            }
                        }
                    });
                }
                Err(e) => {
                    result = Err(RunServerError::Io(e));
                    break;
                }
            }
        }

        result
    })
}

struct Client<'a, T: Read + Write> {
    transport: ClientTransport<T>,
    mutex: &'a Mutex<()>,
}

impl<'a, T: Read + Write> Client<'a, T> {
    pub fn run(io: T, mutex: &'a Mutex<()>) -> Result<(), ProtocolError> {
        Self::new(io, mutex)?.run_client()
    }

    pub fn new(io: T, mutex: &'a Mutex<()>) -> Result<Self, ProtocolError> {
        Ok(Client {
            transport: ClientTransport::new(io)?,
            mutex,
        })
    }

    pub fn run_client(&mut self) -> Result<(), ProtocolError> {
        // self.transport.send(Reply::Hello { version: env!("CARGO_PKG_VERSION") });

        while let Some((message, mut txn)) = self.transport.recv()? {
            match message {
                Message::Ping => txn.reply(ReplyData::Ok)?,
                Message::Check { mut config, files } => {
                    let _ = self.mutex.lock().expect("mutex lock in Client::run_client");

                    if let Some(lib_path) = env::var("TYPEDRUBY_LIB").ok() {
                        config.require_paths.insert(0, PathBuf::from(lib_path));
                    } else {
                        // errors.warning("TYPEDRUBY_LIB environment variable not set, will not use builtin standard library definitions", &[]);
                    }

                    let mut errors = ClientErrors::new(txn);
                    let arena = Arena::new();

                    let env = Environment::new(&arena, &mut errors, config);

                    env.load_files(files.iter());
                    env.define();
                    env.typecheck();
                }
            }
        }

        Ok(())
    }
}

struct ClientErrors<'a, T: Read + Write + 'a> {
    txn: ClientTransaction<'a, T>,
    error_count: usize,
    warning_count: usize,
}

impl<'a, T: Read + Write + 'a> ClientErrors<'a, T> {
    pub fn new(txn: ClientTransaction<'a, T>) -> Self {
        ClientErrors { txn, error_count: 0, warning_count: 0 }
    }
}

fn map_details(details: &[errors::Detail]) -> Vec<protocol::Detail> {
    details.iter().map(|detail| match *detail {
        errors::Detail::Message(msg) =>
            protocol::Detail::Message { msg: msg.to_owned() },

        errors::Detail::Loc(msg, ref loc) =>
            protocol::Detail::Loc {
                msg: msg.to_owned(),
                loc: protocol::Loc {
                    file: loc.file().filename().to_owned(),
                    begin_pos: loc.begin_pos,
                    end_pos: loc.end_pos,
                }
            },
    }).collect()
}

impl<'a, T: Read + Write + 'a> ErrorSink for ClientErrors<'a, T> {
    fn error(&mut self, message: &str, details: &[errors::Detail]) {
        // ignore error here:
        let _ = self.txn.reply(ReplyData::Error {
            msg: message.to_owned(),
            details: map_details(details),
        });
    }

    fn warning(&mut self, message: &str, details: &[errors::Detail]) {
        // ignore error here:
        let _ = self.txn.reply(ReplyData::Warning {
            msg: message.to_owned(),
            details: map_details(details),
        });
    }

    fn error_count(&self) -> usize {
        self.error_count
    }

    fn warning_count(&self) -> usize {
        self.warning_count
    }
}
