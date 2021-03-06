use std::convert::From;
use std::error::Error;
use std::fmt;
use std::io;
use std::net;
use std::str::Utf8Error;

pub type FingerResult<T> = Result<T, FingerError>;

#[derive(Debug)]
pub enum FingerError {
    IoError(io::Error),
    ParseError(String),
    Utf8Error(Utf8Error),
    HostError(net::AddrParseError),
}

impl FingerError {
    pub fn parse<S: Into<String>>(msg: S) -> FingerError {
        FingerError::ParseError(msg.into())
    }
}

impl fmt::Display for FingerError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str("FingerError: ")?;
        fmt.write_str(self.description())?;
        Ok(())
    }
}

impl Error for FingerError {
    fn description(&self) -> &str {
        match *self {
            FingerError::IoError(_) => "IO failure",
            FingerError::ParseError(_) => "Parsing failure",
            FingerError::Utf8Error(_) => "Utf-8 failure",
            FingerError::HostError(_) => "Hostname parsing failure",
        }
    }
    fn cause(&self) -> Option<&Error> {
        match *self {
            FingerError::IoError(ref err) => Some(err),
            FingerError::Utf8Error(ref err) => Some(err),
            FingerError::HostError(ref err) => Some(err),
            // FingerError::ParseError(ref err) => Some(err),
            _ => None,
        }
    }
}

impl From<io::Error> for FingerError {
    fn from(err: io::Error) -> Self {
        FingerError::IoError(err)
    }
}

impl From<Utf8Error> for FingerError {
    fn from(err: Utf8Error) -> Self {
        FingerError::Utf8Error(err)
    }
}

impl From<net::AddrParseError> for FingerError {
    fn from(err: net::AddrParseError) -> Self {
        FingerError::HostError(err)
    }
}
