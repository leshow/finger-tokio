use bytes::BytesMut;
use std::borrow::Borrow;
use std::fmt;
use std::io;
use std::marker::PhantomData;
use std::ops::Deref;
use std::str;
use tokio_io::codec::{Encoder, Decoder};

const DELIM: u8 = b'\n';
const SEPARATOR: char = '@';
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
    fn set_hostname<S: Into<Option<String>>>(self, hostname: S) -> FingerFrame {
        FingerFrame {
            hostname: hostname.into(),
            ..self
        }
    }
    fn set_username<S: Into<Option<String>>>(self, username: S) -> FingerFrame {
        FingerFrame {
            username: username.into(),
            ..self
        }
    }
}


impl fmt::Display for FingerFrame {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        if let Some(ref u) = self.username {
            fmt.write_str(u);
        }
        if let Some(ref h) = self.hostname {
            fmt.write_str("@");
            fmt.write_str(h);
        }
        Ok(())
    }
}

trait Finger {
    fn hostname(&self) -> Option<&str>;
    fn username(&self) -> Option<&str>;
    fn write_to(&self) -> FingerResult<String>;
}

impl Finger for FingerFrame {
    fn hostname(&self) -> Option<&str> {
        self.hostname.as_ref().map(|x| &**x)
    }
    fn username(&self) -> Option<&str> {
        self.username.as_ref().map(|x| &**x)
    }
    fn write_to(&self) -> FingerResult<String> {
        Ok(format!("{}", self))
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

            // right now only handles a single user@host
            let pair = input
                .split(SEPARATOR)
                .map(|x| x.to_owned())
                .collect::<Vec<String>>();
            let mut pair = pair.into_iter();
            let frame = FingerFrame::new()
                .set_username(pair.next())
                .set_hostname(pair.next());

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
    type Error = FingerError;
    fn encode(&mut self, input: Self::Item, buf: &mut BytesMut) -> Result<(), Self::Error> {
        let frame = input.borrow();
        buf.extend_from_slice(frame.write_to()?.as_ref());
        buf.extend(b"\n");
        Ok(())
    }
}