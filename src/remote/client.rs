use std::collections::HashMap;
use std::io::{self, ErrorKind};
use std::os::unix::net::UnixStream;
use std::path::{Path, PathBuf};
use std::rc::Rc;

use typed_arena::Arena;

use ast::{SourceFile, Loc};
use remote::protocol::{self, ServerTransport, ProtocolError, Message, ReplyData};
use report::{self, ErrorSink};

type SourceCache = HashMap<PathBuf, Rc<SourceFile>>;

fn get_source_file(cache: &mut SourceCache, path: &Path) -> Result<Rc<SourceFile>, io::Error> {
    if let Some(file) = cache.get(path) {
        return Ok(Rc::clone(&file));
    }

    let file = Rc::new(SourceFile::open(path.to_owned())?);
    cache.insert(path.to_owned(), Rc::clone(&file));
    Ok(file)
}

fn map_details<'a>(arena: &'a Arena<Loc>, cache: &mut SourceCache, details: &'a [protocol::Detail]) -> Result<Vec<report::Detail<'a>>, io::Error> {
    details.iter().map(|detail| match *detail {
        protocol::Detail::Message { ref msg } =>
            Ok(report::Detail::Message(msg)),

        protocol::Detail::Loc { ref msg, ref loc } => {
            let source_file = get_source_file(cache, &loc.file)?;

            Ok(report::Detail::Loc(msg,
                arena.alloc(Loc::new(source_file, loc.begin_pos, loc.end_pos))))
        }
    }).collect::<Result<Vec<_>, _>>()
}

pub struct Remote {
    transport: ServerTransport<UnixStream>,
}

pub enum ConnectError {
    NoServer,
    VersionMismatch(String),
    Protocol(ProtocolError),
    Io(io::Error),
}

impl Remote {
    pub fn connect(socket_path: &Path) -> Result<Remote, ConnectError> {
        let stream = UnixStream::connect(socket_path).map_err(|e|
            match e.kind() {
                ErrorKind::NotFound |
                ErrorKind::AddrNotAvailable |
                ErrorKind::ConnectionRefused => ConnectError::NoServer,
                _ => ConnectError::Io(e)
            }
        )?;

        let transport = ServerTransport::new(stream).map_err(|e|
            match e {
                ProtocolError::VersionMismatch(version) => ConnectError::VersionMismatch(version),
                _ => ConnectError::Protocol(e),
            }
        )?;

        Ok(Remote { transport })
    }

    pub fn check(&mut self, errors: &mut ErrorSink)
        -> Result<bool, ProtocolError>
    {
        let mut source_cache = SourceCache::new();

        for reply in self.transport.send(Message::Check)? {
            let arena = Arena::new();

            match reply? {
                ReplyData::Error { msg, details } => {
                    let details = map_details(&arena, &mut source_cache, &details).map_err(ProtocolError::Io)?;
                    errors.error(&msg, &details)
                }
                ReplyData::Warning { msg, details } => {
                    let details = map_details(&arena, &mut source_cache, &details).map_err(ProtocolError::Io)?;
                    errors.warning(&msg, &details)
                }
                ReplyData::Ok => {}
            }
        }

        Ok(true)
    }
}
