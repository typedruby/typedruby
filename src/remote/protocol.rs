use serde::{Serialize, Deserialize};
use serde_json;
use std::io::{self, Read, Write, BufRead, BufReader};
use std::iter::Iterator;
use std::ops::Drop;
use std::path::PathBuf;

pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");

#[derive(Debug)]
pub enum ProtocolError {
    Io(io::Error),
    Json(serde_json::Error),
    VersionMismatch(String),
    Violation(&'static str),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "t")]
pub enum Message {
    Ping,
    Check,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "t")]
pub enum Reply {
    Hello { version: String },
    Accepted,
    Data { data: ReplyData },
    End,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "t")]
pub enum ReplyData {
    Error { msg: String, details: Vec<Detail> },
    Warning { msg: String, details: Vec<Detail> },
    Ok,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "t")]
pub enum Detail {
    Message { msg: String },
    Loc { msg: String, loc: Loc },
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Loc {
    pub file: PathBuf,
    pub begin_pos: usize,
    pub end_pos: usize,
}

fn read_json<'a, D: Deserialize<'a>, T: BufRead>(io: &mut T, buff: &'a mut String) -> Result<Option<D>, ProtocolError> {
    match io.read_line(buff) {
        Ok(0) => Ok(None),
        Ok(_) => {
            let msg = serde_json::from_str(buff).map_err(ProtocolError::Json)?;
            Ok(Some(msg))
        }
        Err(e) => Err(ProtocolError::Io(e)),
    }
}

fn write_json<S: Serialize, T: Write>(io: &mut T, data: S) -> Result<(), ProtocolError> {
    let string = serde_json::to_string(&data)
        .map_err(ProtocolError::Json)? + "\n";

    io.write_all(string.as_bytes())
        .map_err(ProtocolError::Io)
}

fn handshake<T: Read + Write>(io: &mut BufReader<T>) -> Result<(), ProtocolError> {
    write_json(io.get_mut(), VERSION)?;

    let mut buff = String::new();
    let remote_version: Option<&str> = read_json(io, &mut buff)?;

    match remote_version {
        Some(ver) if ver == VERSION => {
            Ok(())
        }
        Some(ver) => {
            Err(ProtocolError::VersionMismatch(ver.to_string()))
        }
        None => {
            Err(ProtocolError::Violation("expected version handshake"))
        }
    }
}

pub struct ClientTransport<T: Read + Write> {
    io: BufReader<T>,
}

impl<T: Read + Write> ClientTransport<T> {
    pub fn new(io: T) -> Result<Self, ProtocolError> {
        let mut io = BufReader::new(io);

        handshake(&mut io)?;

        Ok(ClientTransport { io })
    }

    pub fn recv<'a>(&'a mut self) -> Result<Option<(Message, ClientTransaction<'a, T>)>, ProtocolError> {
        let mut buff = String::new();
        let msg = read_json(&mut self.io, &mut buff)?;

        match msg {
            Some(msg) => {
                let txn = ClientTransaction::new(self)?;
                Ok(Some((msg, txn)))
            }
            None => Ok(None)
        }
    }

    fn send_raw(&mut self, reply: Reply) -> Result<(), ProtocolError> {
        write_json(self.io.get_mut(), reply)
    }
}

pub struct ClientTransaction<'a, T: Read + Write + 'a> {
    transport: &'a mut ClientTransport<T>,
}

impl<'a, T: Read + Write> ClientTransaction<'a, T> {
    fn new(transport: &'a mut ClientTransport<T>) -> Result<Self, ProtocolError> {
        transport.send_raw(Reply::Accepted)?;

        Ok(ClientTransaction { transport })
    }

    pub fn reply(&mut self, data: ReplyData) -> Result<(), ProtocolError> {
        self.transport.send_raw(Reply::Data { data })
    }
}

impl<'a, T: Read + Write> Drop for ClientTransaction<'a, T> {
    fn drop(&mut self) {
        // ignore error here?
        let _ = self.transport.send_raw(Reply::End);
    }
}

pub struct ServerTransport<T: Read + Write> {
    io: BufReader<T>,
}

impl<T: Read + Write> ServerTransport<T> {
    pub fn new(io: T) -> Result<Self, ProtocolError> {
        let mut io = BufReader::new(io);

        handshake(&mut io)?;

        Ok(ServerTransport { io })
    }

    fn recv_raw(&mut self) -> Result<Option<Reply>, ProtocolError> {
        let mut buff = String::new();
        Ok(read_json(&mut self.io, &mut buff)?)
    }

    pub fn send<'a>(&'a mut self, msg: Message) -> Result<ServerTransaction<'a, T>, ProtocolError> {
        write_json(self.io.get_mut(), msg)?;

        match self.recv_raw()? {
            Some(Reply::Accepted) => {
                Ok(ServerTransaction::new(self))
            }
            None => Err(ProtocolError::Violation("server closed connection unexpectedly")),
            Some(_) => Err(ProtocolError::Violation("unexpected reply")),
        }
    }
}

pub struct ServerTransaction<'a, T: Read + Write + 'a> {
    transport: &'a mut ServerTransport<T>,
    complete: bool,
}

impl<'a, T: Read + Write + 'a> ServerTransaction<'a, T> {
    fn new(transport: &'a mut ServerTransport<T>) -> Self {
        ServerTransaction { transport, complete: false }
    }
}

impl<'a, T: Read + Write + 'a> Iterator for ServerTransaction<'a, T> {
    type Item = Result<ReplyData, ProtocolError>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.transport.recv_raw() {
            Ok(Some(Reply::Data { data })) => Some(Ok(data)),
            Ok(Some(Reply::End)) => {
                self.complete = true;
                None
            }
            Ok(Some(_)) => {
                self.complete = true;
                Some(Err(ProtocolError::Violation("unexpected reply")))
            }
            Ok(None) => {
                self.complete = true;
                Some(Err(ProtocolError::Violation("server closed connection unexpectedly")))
            }
            Err(e) => Some(Err(e)),
        }
    }
}

impl<'a, T: Read + Write + 'a> Drop for ServerTransaction<'a, T> {
    fn drop(&mut self) {
        if !self.complete {
            while let Some(Ok(_)) = self.next() {}
        }
    }
}
