use std::io;
use std::str;
use std::ops::Deref;
use std::marker::PhantomData;
use std::borrow::Borrow;
use bytes::BytesMut;
use tokio_io::codec::{Encoder, Decoder};

const DELIM: u8 = b'\n';
const PORT_NUM: u16 = 79;


pub struct FingerCodec<F> {
    frame_type: PhantomData<F>
}

struct FingerFrame {
    pub username: Option<String>,
    pub hostname: Option<String>,
}

trait Finger {
    fn hostname(&self) -> Option<&str>;
    fn username(&self) -> Option<&str>;
}

impl Finger for FingerFrame {
    fn hostname(&self) -> Option<&str> {
        self.hostname.as_ref().map(|x| &**x)
    }
    fn username(&self) -> Option<&str> {
        self.username.as_ref().map(Deref::deref)
    }
}

impl<F> FingerCodec<F> {
    fn new() -> FingerCodec<F> {
        FingerCodec {
            frame_type: PhantomData
        }
    }
}

impl FingerCodec<FingerFrame> {
    fn default() -> Self {
        FingerCodec {
            frame_type: PhantomData
        }
    }
}

impl<F> Decoder for FingerCodec<F>
where F: Borrow<Finger> {
    type Item = F;
    type Error = io::Error;
    fn decode(&mut self, buf: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        unimplemented!();
        // if let Some(i) = buf.iter().position(|&b| b == DELIM) {
        //     let line = buf.split_to(i);

        //     buf.split_to(1); // break off '\n'
        //     match str::from_utf8(&line) {
        //         Ok(l) => Ok(Some(String::from(l))),
        //         Err(_) => Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid utf-8")),
        //     }
        // } else {
        //     Ok(None)
        // }
    }
}

impl<F> Encoder for FingerCodec<F> 
where F: Borrow<Finger> {
    type Item = F;
    type Error = io::Error;
    fn encode(&mut self, input: Self::Item, buf: &mut BytesMut) -> Result<(), Self::Error> {
        unimplemented!();
        // buf.extend_from_slice(input.as_bytes());
        // buf.extend(b"\n");
        // Ok(())
    }
}