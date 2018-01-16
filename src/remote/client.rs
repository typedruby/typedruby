use std::collections::HashMap;
use std::io;
use std::os::unix::net::UnixStream;
use std::path::{Path, PathBuf};
use std::rc::Rc;

use typed_arena::Arena;

use ast::{SourceFile, Loc};
use config::CheckConfig;
use errors::{self, ErrorSink};
use remote::protocol::{self, ServerTransport, ProtocolError, Message, ReplyData};

type SourceCache = HashMap<PathBuf, Rc<SourceFile>>;

fn get_source_file(cache: &mut SourceCache, path: &Path) -> Result<Rc<SourceFile>, io::Error> {
    if let Some(file) = cache.get(path) {
        return Ok(Rc::clone(&file));
    }

    let file = Rc::new(SourceFile::open(path.to_owned())?);
    cache.insert(path.to_owned(), Rc::clone(&file));
    Ok(file)
}

fn map_details<'a>(arena: &'a Arena<Loc>, cache: &mut SourceCache, details: &'a [protocol::Detail]) -> Result<Vec<errors::Detail<'a>>, io::Error> {
    details.iter().map(|detail| match *detail {
        protocol::Detail::Message { ref msg } =>
            Ok(errors::Detail::Message(msg)),

        protocol::Detail::Loc { ref msg, ref loc } => {
            let source_file = get_source_file(cache, &loc.file)?;

            Ok(errors::Detail::Loc(msg,
                arena.alloc(Loc::new(source_file, loc.begin_pos, loc.end_pos))))
        }
    }).collect::<Result<Vec<_>, _>>()
}

pub fn check_remote(socket_path: &Path, errors: &mut ErrorSink, config: CheckConfig, files: Vec<PathBuf>)
    -> Result<bool, ProtocolError>
{
    let stream = UnixStream::connect(socket_path)
        .map_err(ProtocolError::Io)?;

    let mut transport = ServerTransport::new(stream)?;

    let mut source_cache = SourceCache::new();

    for reply in transport.send(Message::Check { config, files })? {
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
