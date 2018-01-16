use std::os::unix::net::UnixStream;
use std::path::{Path, PathBuf};

use config::CheckConfig;
use errors::ErrorSink;
use protocol::{ServerTransport, ProtocolError, Message};

pub fn check_remote(socket_path: &Path, errors: &mut ErrorSink, config: CheckConfig, files: Vec<PathBuf>)
    -> Result<bool, ProtocolError>
{
    let stream = UnixStream::connect(socket_path)
        .map_err(ProtocolError::Io)?;

    let mut transport = ServerTransport::new(stream)?;

    for reply in transport.send(Message::Check { config, files })? {
        println!("reply: {:?}", reply?);
    }

    Ok(true)
}
