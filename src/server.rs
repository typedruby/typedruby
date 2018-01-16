use std::env;
use std::io::{self, Read, Write, BufRead, BufReader};
use std::os::unix::net::UnixListener;
use std::path::PathBuf;
use std::sync::mpsc::{self, Receiver, SyncSender};

use protocol::{ProtocolError, Message, Reply, ClientTransport, ReplyData};

use crossbeam;

#[derive(Debug)]
pub enum RunServerError {
    Io(io::Error)
}

pub fn run() -> Result<(), RunServerError> {
    let path = socket_path()?;

    let listener = UnixListener::bind(&path).map_err(RunServerError::Io)?;

    let (send, recv) = mpsc::sync_channel(0);

    crossbeam::scope(|scope| {
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

pub fn socket_path() -> Result<PathBuf, RunServerError> {
    let mut path = env::current_dir().map_err(RunServerError::Io)?;

    path.push(".typedruby.sock");

    Ok(path)
}

struct Client<T: Read + Write> {
    transport: ClientTransport<T>,
    send: SyncSender<DispatchEvent>,
}

impl<T: Read + Write> Client<T> {
    pub fn run(io: T, send: SyncSender<DispatchEvent>) -> Result<(), ProtocolError> {
        Self::new(io, send)?.run_client()
    }

    pub fn new(io: T, send: SyncSender<DispatchEvent>) -> Result<Self, ProtocolError> {
        Ok(Client {
            transport: ClientTransport::new(io)?,
            send,
        })
    }

    pub fn run_client(&mut self) -> Result<(), ProtocolError> {
        // self.transport.send(Reply::Hello { version: env!("CARGO_PKG_VERSION") });

        while let Some(mut txn) = self.transport.recv()? {
            match *txn.message() {
                Message::Ping => txn.reply(ReplyData::Ok)?,
                Message::Check { .. } => {
                    println!("implement check! {:?}", txn.message());
                }
            }
        }

        Ok(())
    }
}

enum DispatchEvent {

}

fn dispatch(recv: Receiver<DispatchEvent>) {

}
