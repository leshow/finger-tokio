extern crate bytes;
extern crate futures;
extern crate futures_cpupool;
extern crate tokio_io;
extern crate tokio_proto;
extern crate tokio_service;
extern crate tokio_core;

mod proto;
mod error;

pub use error::{FingerError, FingerResult};

use futures::{BoxFuture, Future, future};
use futures_cpupool::CpuPool;
pub use proto::{Finger, FingerCodec, FingerFrame, PORT_NUM};

use std::fs::File;
use std::io::{self, BufRead, BufReader};
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


pub struct FingerService {
    thread_pool: CpuPool,
}

impl Service for FingerService {
    type Request = FingerFrame;
    type Response = FingerFrame;
    type Error = io::Error;
    type Future = BoxFuture<Self::Response, Self::Error>; // response future

    fn call(&self, req: Self::Request) -> Self::Future {
        let query = self.thread_pool.spawn_fn(move || {
            let frame = match req.hostname() {
                Some(host) => {
                    match req.username() {
                        Some(user) => {
                            // host && user
                            let res = query_local(user)
                                .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

                        }
                        None => {
                            // err
                        }
                    }
                }
                None => {}
            };

            Ok(req)
        });
        query.boxed()
        //        future::ok(req).boxed()
    }
}

struct Entry {
    pub name:  String,
    pub home:  String,
    pub shell: String,
    pub gecos: Option<Gecos>,
}

struct Gecos {
    pub full_name: String,
    pub location:  String,
    pub phone:     String,
    pub other:     Vec<String>,
}

fn query_local(username: &str) -> FingerResult<Entry> {
    let f = File::open("/etc/passwd")?;
    let reader = BufReader::new(f);
    let username = username.to_lowercase();

    let lines = reader.lines();
    for line in lines {
        let entry = parse_line(line?)?;
        if entry.name.to_lowercase() == username {
            return Ok(entry);
        }
    }
    Err(FingerError::parse("No user found"))
}

fn parse_line(line: String) -> FingerResult<Entry> {
    let mut user = line.split(':');
    let name = parse_part(&mut user, "/cat/passwd: Name not found")?;
    user.next();
    user.next();
    user.next();
    let part = parse_part(&mut user, "gecos not found");

    let gecos = match part {
        Ok(p) => Some(parse_gecos(p)?),
        Err(_) => None,
    };

    let home = parse_part(&mut user, "/cat/passwd: Home not found")?;
    let shell = parse_part(&mut user, "/cat/passwd: Shell not found")?;

    Ok(Entry {
        name,
        gecos,
        home,
        shell,
    })
}

fn parse_gecos(line: String) -> FingerResult<Gecos> {
    let mut gecos = line.split(',');
    let full_name = parse_part(&mut gecos, "Gecos: full name parse failed")?;
    let location = parse_part(&mut gecos, "Gecos: location parse failed")?;
    let phone = parse_part(&mut gecos, "Gecos: phone parse failed")?;

    let other = gecos.map(|s| s.to_owned()).collect::<Vec<String>>();

    Ok(Gecos {
        full_name,
        location,
        phone,
        other,
    })
}

// `mut user` works as well as `user: &mut I` here
// because: impl<'a, T: Iterator> Iterator for &'a mut T
fn parse_part<'a, I, S>(mut part: I, e: S) -> FingerResult<String>
where
    I: Iterator<Item = &'a str>,
    S: Into<String>,
{
    Ok(part.next().ok_or(FingerError::parse(e))?.to_owned())
}

fn main() {
    let addr = format!("0.0.0.0:12345").parse().unwrap();
    let server = TcpServer::new(FingerProto, addr);
    server.serve(|| {
        Ok(FingerService {
            thread_pool: CpuPool::new(4),
        })
    });
}
