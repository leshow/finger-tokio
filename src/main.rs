extern crate bytes;
extern crate futures;
extern crate tokio_io;
extern crate tokio_proto;
extern crate tokio_service;
extern crate tokio_core;

mod proto;
mod error;

use futures::{BoxFuture, Future, future};
pub use proto::{Finger, FingerCodec, FingerFrame, PORT_NUM};

use std::io;
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
        future::ok(req).boxed()
    }
}


fn main() {
    let addr = format!("0.0.0.0:12345").parse().unwrap();
    let server = TcpServer::new(FingerProto, addr);
    server.serve(|| Ok(FingerService));
}
