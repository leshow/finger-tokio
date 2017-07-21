use bytes::BytesMut;
use std::fmt;
use std::io;
use std::str;
use tokio_io::codec::{Decoder, Encoder};

pub const DELIM: u8 = b'\n';
pub const SEPARATOR: char = '@';
pub const PORT_NUM: u16 = 79;

use error::FingerResult;

pub struct FingerCodec;

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
}

impl fmt::Display for FingerFrame {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        if let Some(ref u) = self.username {
            fmt.write_str(u)?;
            if let Some(ref h) = self.hostname {
                fmt.write_str("@")?;
                fmt.write_str(h)?;
            }
        }
        Ok(())
    }
}

pub trait Finger {
    fn hostname(&self) -> Option<&str>;
    fn set_hostname<S: Into<Option<String>>>(&mut self, hostname: S);
    fn set_username<S: Into<Option<String>>>(&mut self, username: S);
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
    fn set_hostname<S: Into<Option<String>>>(&mut self, hostname: S) {
        self.hostname = hostname.into();
    }
    fn set_username<S: Into<Option<String>>>(&mut self, username: S) {
        self.username = username.into();
    }
}

impl Decoder for FingerCodec {
    type Item = FingerFrame;
    type Error = io::Error;

    fn decode(&mut self, buf: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if let Some(i) = buf.iter().position(|&b| b == DELIM) {
            let line = buf.split_to(i);
            buf.split_to(1); // break off '\n'
            let input = str::from_utf8(&line)
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
            // right now only handles a single user@host
            let pair = input
                .split(SEPARATOR)
                .map(|x| x.to_owned())
                .collect::<Vec<String>>();
            let mut pair = pair.into_iter();
            let mut frame = FingerFrame::new();
            frame.set_username(pair.next());
            frame.set_hostname(pair.next());

            Ok(Some(frame))
        } else {
            Ok(None)
        }
    }
}

impl Encoder for FingerCodec {
    type Item = FingerFrame;
    type Error = io::Error;

    fn encode(&mut self, input: Self::Item, buf: &mut BytesMut) -> Result<(), Self::Error> {
        buf.extend_from_slice(
            input
                .write_to()
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?
                .as_ref(),
        );
        buf.extend(b"\n");
        Ok(())
    }
}
