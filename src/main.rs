extern crate bytes;
extern crate futures;
extern crate tokio_io;
extern crate tokio_proto;
extern crate tokio_service;
extern crate tokio_core;

mod proto;
mod error;

use error::{FingerError, FingerResult};
use proto::{Finger, FingerCodec, FingerFrame};
use std::borrow::Borrow;
use std::io;
// use tokio_core::io::Framed;
use tokio_io::{AsyncRead, AsyncWrite};
use tokio_io::codec::Framed;
use tokio_proto::pipeline::ServerProto;

pub struct FingerProto<F> {
    frame: std::marker::PhantomData<F>,
}

impl<T, F> ServerProto<T> for FingerProto<F>
where
    T: AsyncRead + AsyncWrite,
    F: Borrow<Finger>,
{
    type Request = FingerFrame;
    type Response = FingerFrame; // matches Item type in Decoder
    type Transport = Framed<T, FingerCodec<FingerFrame>>;
    type BindTransport = Result<Self::Transport, FingerError>;

    fn bind_transport(&self, io: T) -> Self::BindTransport {
        unimplemented!();
        //         Ok(io.framed(FingerCodec::default()));
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

// use tokio_proto::TcpServer;

fn main() {
    // let addr = "0.0.0.0:12345".parse().unwrap();
    // let server = TcpServer::new(FingerProto, addr);
    // server.serve(|| Ok(EchoRev));
}
