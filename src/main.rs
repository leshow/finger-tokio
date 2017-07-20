extern crate bytes;
extern crate futures;
extern crate tokio_io;
extern crate tokio_proto;
extern crate tokio_service;
extern crate tokio_core;

mod proto;
mod error;

use error::{FingerError, FingerResult};
use futures::{BoxFuture, Future, future};
pub use proto::{Finger, FingerCodec, FingerFrame, PORT_NUM};
use std::fs::File;
use std::io::{self, BufReader};


use std::io::prelude::*;
use tokio_io::{AsyncRead, AsyncWrite};
use tokio_io::codec::Framed;
use tokio_proto::TcpServer;
use tokio_proto::pipeline::ServerProto;
use tokio_service::Service;

pub struct FingerProto;

impl<T> ServerProto<T> for FingerProto
where
    T: AsyncRead + AsyncWrite + 'static,
{
    type Request = FingerFrame;
    type Response = FingerFrame; // matches Item type in Decoder
    type Transport = Framed<T, FingerCodec>;
    type BindTransport = Result<Self::Transport, io::Error>;

    fn bind_transport(&self, io: T) -> Self::BindTransport {
        Ok(io.framed(FingerCodec))
    }
}


pub struct FingerService;

impl Service for FingerService {
    type Request = FingerFrame;
    type Response = FingerFrame;
    type Error = io::Error;
    type Future = BoxFuture<Self::Response, Self::Error>; // response future

    fn call(&self, req: Self::Request) -> Self::Future {
        if let Some(host) = req.hostname() {
            if host == "localhost" {
                // local
            } else {
                // remote
            }
        } else {
            // local
        }
        future::ok(req).boxed()
    }
}

struct Entry {
    pub name:  String,
    pub home:  String,
    pub shell: String,
}

fn query_local(username: &str) -> FingerResult<Entry> {
    let mut f = File::open("/etc/passwd")?;
    let mut reader = BufReader::new(f);
    let username = username.to_lowercase();

    let lines = reader.lines();

    // lines
    //     .filter_map(Result::ok)
    //     .map(parse_line)
    //     .filter_map(Result::ok)
    //     .map(|entry| entry.name.to_lowercase())
    //     .filter(|name| *name == username)
    //     .collect::<Vec<String>>();

    for line in lines {
        let entry = parse_line(line?)?;
        if entry.name.to_lowercase() == username {
            return Ok(entry);
        }
    }
    Err(FingerError::parse("No user found"))
}

fn parse_line<'a>(line: String) -> FingerResult<Entry> {
    let mut user = line.split(':');
    let name = user.next()
        .ok_or(FingerError::parse("/cat/passwd: Name not found"))?
        .to_owned();
    //    let name = get_entry(&mut user, "/cat/passwd: Name not found")?;

    user.next();
    user.next();
    user.next();
    user.next();

    let home = user.next()
        .ok_or(FingerError::parse("/cat/passwd: Home not found"))?
        .to_owned();
    let shell = user.next()
        .ok_or(FingerError::parse("/cat/passwd: Shell not found"))?
        .to_owned();
    Ok(Entry { name, home, shell })
}

fn get_entry<'a, I: Iterator<Item = &'a str>, S: Into<String>>(
    user: &mut I,
    e: S,
) -> FingerResult<String> {
    Ok(user.next().ok_or(FingerError::parse(e))?.to_owned())
}


fn main() {
    let addr = format!("0.0.0.0:12345").parse().unwrap();
    let server = TcpServer::new(FingerProto, addr);
    server.serve(|| Ok(FingerService));
}
