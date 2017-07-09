use bytes::BytesMut;
use std::borrow::Borrow;
use std::io;
use std::marker::PhantomData;
use std::ops::Deref;
use std::str;
use tokio_io::codec::{Encoder, Decoder};

const DELIM: u8 = b'\n';
const PORT_NUM: u16 = 79;

use error::{FingerResult, FingerError};

pub struct FingerCodec<F> {
    frame_type: PhantomData<F>,
}

pub struct FingerFrame {
    pub username: Option<String>,
    pub hostname: Option<String>,
}

impl FingerFrame {
    fn new() -> FingerFrame {
        FingerFrame {
            username: None,
            hostname: None,
        }
    }
    fn set_hostname<S: Into<String>>(self, hostname: S) -> FingerFrame {
        FingerFrame {
            hostname: Some(hostname.into()),
            ..self
        }
    }
    fn set_username<S: Into<String>>(self, username: S) -> FingerFrame {
        FingerFrame {
            username: Some(username.into()),
            ..self
        }
    }
}

trait Finger {
    fn hostname(&self) -> Option<&str>;
    fn username(&self) -> Option<&str>;
    fn write_to(&self) -> FingerResult<()>;
}

impl Finger for FingerFrame {
    fn hostname(&self) -> Option<&str> {
        self.hostname.as_ref().map(|x| &**x)
    }
    fn username(&self) -> Option<&str> {
        self.username.as_ref().map(Deref::deref)
    }
    fn write_to(&self) -> FingerResult<()> {
        Ok(())
    }
}

impl<F> FingerCodec<F> {
    fn new() -> FingerCodec<F> {
        FingerCodec {
            frame_type: PhantomData,
        }
    }
}

impl FingerCodec<FingerFrame> {
    fn default() -> Self {
        FingerCodec {
            frame_type: PhantomData,
        }
    }
}

impl<F> Decoder for FingerCodec<F> {
    type Item = FingerFrame;
    type Error = FingerError;
    fn decode(&mut self, buf: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if let Some(i) = buf.iter().position(|&b| b == DELIM) {
            let line = buf.split_to(i);

            buf.split_to(1); // break off '\n'
            let input = str::from_utf8(&line)?;


            let frame = FingerFrame::new();

            Ok(Some(frame))
        } else {
            Ok(None)
        }
    }
}

impl<F> Encoder for FingerCodec<F>
where
    F: Borrow<Finger>,
{
    type Item = F;
    type Error = io::Error;
    fn encode(&mut self, input: Self::Item, buf: &mut BytesMut) -> Result<(), Self::Error> {
        unimplemented!();
        // buf.extend_from_slice(input.as_bytes());
        // buf.extend(b"\n");
        // Ok(())
    }
}