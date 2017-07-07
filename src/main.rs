extern crate bytes;
extern crate futures;
extern crate tokio_io;
extern crate tokio_proto;
extern crate tokio_service;

use std::io;
use std::str;
use bytes::BytesMut;
use tokio_io::codec::{Encoder, Decoder};


pub struct FingerCodec;

const DELIM: u8 = b'\n';

impl Decoder for LineCodec {
    type Item = String;
    type Error = io::Error;
    fn decode(&mut self, buf: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if let Some(i) = buf.iter().position(|&b| b == DELIM) {
            let line = buf.split_to(i);

            buf.split_to(1); // break off '\n'
            match str::from_utf8(&line) {
                Ok(l) => Ok(Some(String::from(l))),
                Err(_) => Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid utf-8")),
            }
        } else {
            Ok(None)
        }
    }
}

impl Encoder for LineCodec {
    type Item = String;
    type Error = io::Error;
    fn encode(&mut self, input: Self::Item, buf: &mut BytesMut) -> Result<(), Self::Error> {

        buf.extend_from_slice(input.as_bytes());
        buf.extend(b"\n");
        Ok(())
    }
}

use tokio_io::{AsyncRead, AsyncWrite};
use tokio_io::codec::Framed;
use tokio_proto::pipeline::ServerProto;

pub struct FingerProto;

impl<T: AsyncRead + AsyncWrite + 'static> ServerProto<T> for FingerProto {
    type Request = String; // matches Item type in Encoder
    type Response = String; // matches Item type in Decoder
    type Transport = Framed<T, LineCodec>;
    type BindTransport = Result<Self::Transport, io::Error>;
    fn bind_transport(&self, io: T) -> Self::BindTransport {
        Ok(io.framed(LineCodec)) // framed<T: Encoder + Decoder>(self, codec: T) -> Framed<Self, T>;
    }
}

use tokio_service::Service;

pub struct Echo;

use futures::{future, Future, BoxFuture};

impl Service for Echo {
    // these match protocol types
    type Request = String;
    type Response = String;

    type Error = io::Error;
    type Future = BoxFuture<Self::Response, Self::Error>; // response future
    fn call(&self, req: Self::Request) -> Self::Future {
        future::ok(req).boxed()
    }
}

pub struct EchoRev;

impl Service for EchoRev {
    type Request = String;
    type Response = String;

    type Error = io::Error;
    type Future = BoxFuture<Self::Response, Self::Error>;
    fn call(&self, req: Self::Request) -> Self::Future {
        let rev = req.chars().rev().collect::<String>();
        future::ok(rev).boxed()
    }
}
use tokio_proto::TcpServer;

fn main() {
    let addr = "0.0.0.0:12345".parse().unwrap();
    let server = TcpServer::new(FingerProto, addr);
    server.serve(|| Ok(EchoRev));
}

