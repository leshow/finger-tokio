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

#[derive(Debug, Clone)]
pub struct FingerRequest {
    pub username: Option<String>,
    pub hostname: Option<String>,
}

impl FingerRequest {
    fn new() -> FingerRequest {
        FingerRequest {
            username: None,
            hostname: None,
        }
    }
}

impl fmt::Display for FingerRequest {
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

impl Finger for FingerRequest {
    fn hostname(&self) -> Option<&str> {
        self.hostname.as_ref().map(|x| &**x)
    }
    fn username(&self) -> Option<&str> {
        self.username.as_ref().map(|x| &**x)
    }
    fn write_to(&self) -> FingerResult<String> {
        Ok(format!("{}", self)) // this should be the part that does the query and what-not
    }
    fn set_hostname<S: Into<Option<String>>>(&mut self, hostname: S) {
        self.hostname = hostname.into();
    }
    fn set_username<S: Into<Option<String>>>(&mut self, username: S) {
        self.username = username.into();
    }
}

impl Decoder for FingerCodec {
    type Item = FingerRequest;
    type Error = io::Error;

    fn decode(&mut self, buf: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if let Some(i) = buf.iter().position(|&b| b == DELIM) {
            let line = buf.split_to(i);
            buf.split_to(1); // break off '\n'
            let input = str::from_utf8(&line)
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
            // right now only handles a single user@host
            let pair = input
                .trim()
                .split(SEPARATOR)
                .map(|x| x.to_owned())
                .collect::<Vec<String>>();
            let mut pair = pair.into_iter();
            let mut req = FingerRequest::new();
            req.set_username(pair.next());
            req.set_hostname(pair.next());

            Ok(Some(req))
        } else {
            Ok(None)
        }
    }
}

#[derive(Debug)]
pub struct Entry {
    pub name:  String,
    pub home:  String,
    pub shell: String,
    pub gecos: Option<Gecos>,
}

#[derive(Debug)]
pub struct Gecos {
    pub full_name: String,
    pub location:  String,
    pub phone:     String,
    pub other:     Vec<String>,
}

// #[derive(Debug)]
// pub struct FingerResponse {
//     pub entry: Option<Entry>,
// }

#[derive(Debug)]
pub enum FingerResponse {
    // a remote response probably doesn't conform to our exact expectations
    // for Entry, so we can return a String here.
    Remote(String),
    Local(Option<Entry>),
}

impl FingerResponse {
    pub fn remote(body: String) -> FingerResponse {
        FingerResponse::Remote(body)
    }
    pub fn local() -> FingerResponse {
        FingerResponse::Local(None)
    }
}

impl Entry {
    pub fn to_resp(self) -> String {
        let gecos = self.gecos.map_or_else(
            || "".to_owned(),
            |gecos| {
                format!(
                    "Real Name: {}\r\nLocation: {}\r\nPhone: {}\r\nOther: {}\r\n",
                    gecos.full_name,
                    gecos.location,
                    gecos.phone,
                    gecos.other.join("\r\n")
                )
            },
        );

        format!(
            "Login Name: {}\r\nHome: {}\r\nShell: {} \r\n{}",
            self.name,
            self.home,
            self.shell,
            gecos
        )
    }
}

impl Encoder for FingerCodec {
    type Item = FingerResponse;
    type Error = io::Error;

    fn encode(&mut self, input: Self::Item, buf: &mut BytesMut) -> Result<(), Self::Error> {
        println!("{:?}", input);
        let content = match input {
            FingerResponse::Remote(s) => s,
            FingerResponse::Local(opt_entry) => opt_entry.map_or_else(
                || "Unable to find user.".to_owned(),
                |entry| entry.to_resp(),
            ),
        };
        buf.extend_from_slice(content.as_bytes());
        buf.extend(b"\n");
        Ok(())
    }
}
