use std::convert::From;
use std::error::Error;
use std::fmt;
use std::io;
use std::str::Utf8Error;

pub type FingerResult<T> = Result<T, FingerError>;

#[derive(Debug)]
pub enum FingerError {
    IoError(io::Error),
    ParseError(&'static str),
    Utf8Error(Utf8Error),
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
        }
    }
    fn cause(&self) -> Option<&Error> {
        match *self {
            FingerError::IoError(ref err) => Some(err),
            FingerError::Utf8Error(ref err) => Some(err),
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