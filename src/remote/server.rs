use std::io::{self, Read, Write};
use std::os::unix::net::UnixListener;
use std::sync::mpsc::{self, SyncSender};

use environment::Environment;
use errors::{self, ErrorSink};
use load::LoadCache;
use remote::protocol::{self, ProtocolError, Message, ClientTransport, ReplyData};
use remote;

use crossbeam;
use typed_arena::Arena;

#[derive(Debug)]
pub enum RunServerError {
    Io(io::Error)
}

type Work = (Message, SyncSender<ReplyData>);

pub fn run() -> Result<(), RunServerError> {
    let path = remote::socket_path().map_err(RunServerError::Io)?;

    let listener = UnixListener::bind(&path).map_err(RunServerError::Io)?;

    let (send, recv) = mpsc::sync_channel::<Work>(0);

    crossbeam::scope(|scope| {
        scope.spawn(move || {
            let load_cache = LoadCache::new();

            while let Ok((message, reply)) = recv.recv() {
                match message {
                    Message::Ping => {
                        let _ = reply.send(ReplyData::Ok);
                    }
                    Message::Check { mut config, files } => {
                        let mut errors = ClientErrors::new(reply);
                        let arena = Arena::new();

                        let env = Environment::new(&arena, &load_cache, &mut errors, config);

                        env.load_files(files.iter());
                        env.define();
                        env.typecheck();
                    }
                }
            }
        });

        let mut result = Ok(());

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    let send = send.clone();
                    scope.spawn(move || {
                        match Client::run(stream, send) {
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

struct Client<T: Read + Write> {
    transport: ClientTransport<T>,
    send_work: SyncSender<Work>,
}

impl<T: Read + Write> Client<T> {
    pub fn run(io: T, send_work: SyncSender<Work>) -> Result<(), ProtocolError> {
        Self::new(io, send_work)?.run_client()
    }

    pub fn new(io: T, send_work: SyncSender<Work>) -> Result<Self, ProtocolError> {
        Ok(Client {
            transport: ClientTransport::new(io)?,
            send_work,
        })
    }

    pub fn run_client(&mut self) -> Result<(), ProtocolError> {
        while let Some((message, mut txn)) = self.transport.recv()? {
            let (reply_send, reply_recv) = mpsc::sync_channel(0);

            let _ = self.send_work.send((message, reply_send));

            while let Ok(reply) = reply_recv.recv() {
                txn.reply(reply)?;
            }
        }

        Ok(())
    }
}

struct ClientErrors {
    reply: SyncSender<ReplyData>,
    error_count: usize,
    warning_count: usize,
}

impl ClientErrors {
    pub fn new(reply: SyncSender<ReplyData>) -> Self {
        ClientErrors { reply, error_count: 0, warning_count: 0 }
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

impl ErrorSink for ClientErrors {
    fn error(&mut self, message: &str, details: &[errors::Detail]) {
        let _ = self.reply.send(ReplyData::Error {
            msg: message.to_owned(),
            details: map_details(details),
        });
    }

    fn warning(&mut self, message: &str, details: &[errors::Detail]) {
        let _ = self.reply.send(ReplyData::Warning {
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
