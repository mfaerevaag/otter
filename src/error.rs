use std::fmt;
use std::error::Error as StdError;

use ws;

#[derive(Debug, Clone)]
pub enum Error {
    Internal(String),

    UnknownNick(String),
}

impl Error {
    pub fn new(err: Error) -> ws::Error {
        ws::Error::new(
            ws::ErrorKind::Custom(Box::new(err.clone())),
            err.description().to_string())
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Internal(_)    => "unexpected server error",
            Error::UnknownNick(_) => "unknown nick",
        }
    }

    fn cause(&self) -> Option<&StdError> {
        None
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Internal(ref s)    => write!(f, "{}: {}", self.description(), s),
            Error::UnknownNick(ref s) => write!(f, "{}: {}", self.description(), s),
        }
    }
}
