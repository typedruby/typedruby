use std::env;
use std::io;
use std::path::PathBuf;

pub mod client;
pub mod protocol;
pub mod server;

pub fn socket_path() -> Result<PathBuf, io::Error> {
    let mut path = env::current_dir()?;

    path.push(".typedruby.sock");

    Ok(path)
}
