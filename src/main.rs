extern crate bytes;
extern crate futures;
extern crate tokio_io;
extern crate tokio_proto;
extern crate tokio_service;

mod proto;
mod error;

use proto::{FingerCodec, FingerFrame};
use error::{FingerResult, FingerError};
use std::borrow::Borrow;
use std::io;
use tokio_io::{AsyncRead, AsyncWrite};
use tokio_io::codec::Framed;
use tokio_proto::pipeline::ServerProto;

pub struct FingerProto;

impl<T: AsyncRead + AsyncWrite + 'static> ServerProto<T> for FingerProto {

    type Request = FingerFrame; // matches Item type in Encoder
    type Response = FingerFrame; // matches Item type in Decoder
    type Transport = Framed<T, FingerCodec<FingerFrame>>;
    type BindTransport = Result<Self::Transport, io::Error>;
    
    fn bind_transport(&self, io: T) -> Self::BindTransport {
        Ok(io.framed(FingerCoded::default())) // framed<T: Encoder + Decoder>(self, codec: T) -> Framed<Self, T>;
    }
}

// use tokio_service::Service;

// pub struct Echo;

// use futures::{future, Future, BoxFuture};

// impl Service for Echo {
//     // these match protocol types
//     type Request = String;
//     type Response = String;

//     type Error = io::Error;
//     type Future = BoxFuture<Self::Response, Self::Error>; // response future
//     fn call(&self, req: Self::Request) -> Self::Future {
//         future::ok(req).boxed()
//     }
// }

// pub struct EchoRev;

// impl Service for EchoRev {
//     type Request = String;
//     type Response = String;

//     type Error = io::Error;
//     type Future = BoxFuture<Self::Response, Self::Error>;
//     fn call(&self, req: Self::Request) -> Self::Future {
//         let rev = req.chars().rev().collect::<String>();
//         future::ok(rev).boxed()
//     }
// }
// use tokio_proto::TcpServer;

fn main() {
    // let addr = "0.0.0.0:12345".parse().unwrap();
    // let server = TcpServer::new(FingerProto, addr);
    // server.serve(|| Ok(EchoRev));
}
