use std::env;
use std::fs;
use std::io::{self, Read, Write};
use std::os::unix::net::{UnixStream, UnixListener};
use std::path::{Path, PathBuf};
use std::sync::mpsc::{self, SyncSender};

use environment::Environment;
use project::{Project, ProjectPath, ProjectError};
use remote::protocol::{self, ProtocolError, Message, ClientTransport, ReplyData};
use report::{self, Reporter};

use crossbeam;
use typed_arena::Arena;

#[derive(Debug)]
pub enum RunServerError {
    NoProjectConfig,
    Io(io::Error),
    Project(ProjectError),
    AlreadyRunning(PathBuf),
}

enum Work {
    Connection(UnixStream, SyncSender<Work>),
    Message(Message, SyncSender<ReplyData>),
}

fn bind_socket(path: &Path) -> Result<UnixListener, RunServerError> {
    // try to bind to the socket eagerly:
    match UnixListener::bind(path) {
        Ok(listener) => { return Ok(listener); }
        Err(_) => {}
    }

    // if that fails, try to connect to it - a server instance might
    // already be running:
    match UnixStream::connect(path) {
        Ok(_) => { return Err(RunServerError::AlreadyRunning(path.to_owned())); }
        Err(_) => {}
    }

    // if that fails, try to remove the socket and make a last-ditch
    // effort to bind again before giving up:
    let _ = fs::remove_file(path);
    UnixListener::bind(path).map_err(RunServerError::Io)
}

pub fn run(reporter: &mut Reporter) -> Result<(), RunServerError> {
    let current_dir = env::current_dir().expect("env::current_dir");

    let project_path = ProjectPath::find(current_dir).ok_or(RunServerError::NoProjectConfig)?;

    let mut project = Project::new(reporter, project_path.clone()).map_err(RunServerError::Project)?;

    reporter.info("Ready");

    let listener = bind_socket(&project.path.socket_path())?;

    let (send, recv) = mpsc::sync_channel::<Work>(0);

    crossbeam::scope(|scope| {
        scope.spawn(move || {
            for stream in listener.incoming() {
                match stream {
                    Ok(stream) => {
                        let stream_sender = send.clone();
                        let _ = send.send(Work::Connection(stream, stream_sender));
                    }
                    Err(_) => { break; }
                }
            }
        });

        while let Ok(work) = recv.recv() {
            match work {
                Work::Connection(stream, send) => {
                    scope.spawn(move || {
                        let _ = Client::run(stream, send);
                    });
                }
                Work::Message(Message::Ping, reply) => {
                    let _ = reply.send(ReplyData::Ok);
                }
                Work::Message(Message::Check, reply) => {
                    let mut reporter = ClientReporter::new(reply);
                    let arena = Arena::new();

                    if project.needs_reload() {
                        reporter.info("Detected change to TypedRuby.toml, reloading project");
                        project = Project::new(&mut reporter, project_path.clone())
                            .map_err(RunServerError::Project)?;
                    }

                    project.refresh(&mut reporter).map_err(RunServerError::Project)?;

                    Environment::new(&arena, &project, &mut reporter).run();
                }
            }
        }

        Ok(())
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

            let _ = self.send_work.send(Work::Message(message, reply_send));

            while let Ok(reply) = reply_recv.recv() {
                txn.reply(reply)?;
            }
        }

        Ok(())
    }
}

struct ClientReporter {
    reply: SyncSender<ReplyData>,
    error_count: usize,
    warning_count: usize,
}

impl ClientReporter {
    pub fn new(reply: SyncSender<ReplyData>) -> Self {
        ClientReporter { reply, error_count: 0, warning_count: 0 }
    }
}

fn map_details(details: &[report::Detail]) -> Vec<protocol::Detail> {
    details.iter().map(|detail| match *detail {
        report::Detail::Message(msg) =>
            protocol::Detail::Message { msg: msg.to_owned() },

        report::Detail::Loc(msg, ref loc) =>
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

impl Reporter for ClientReporter {
    fn info(&mut self, message: &str) {
        let _ = self.reply.send(ReplyData::Info {
            msg: message.to_owned(),
        });
    }

    fn success(&mut self, message: &str) {
        let _ = self.reply.send(ReplyData::Success {
            msg: message.to_owned(),
        });
    }

    fn error(&mut self, message: &str, details: &[report::Detail]) {
        let _ = self.reply.send(ReplyData::Error {
            msg: message.to_owned(),
            details: map_details(details),
        });
    }

    fn warning(&mut self, message: &str, details: &[report::Detail]) {
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
