use std::fmt;
use std::error::Error as StdError;

use ws;

#[derive(Debug, Clone)]
pub enum Error {
    // Internal(String),
    UnsupportedFormat(String),
    UnknownCommand(String),
    WrongNumArgs(String, u8),
    UnknownNick(String),
}

pub fn boxed(err: Error) -> ws::Error {
    ws::Error::new(
        ws::ErrorKind::Custom(Box::new(err.clone())),
        err.description().to_string())
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::UnsupportedFormat(_)    => "unsupported format",
            Error::UnknownCommand(_)    => "unknown command",
            Error::WrongNumArgs(..)    => "wrong number of arguments",
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
            Error::UnsupportedFormat(ref format) =>
                write!(f, "format '{}' unsupported", format),
            Error::UnknownCommand(ref cmd) =>
                write!(f, "{} '{}'", self.description(), cmd),
            Error::WrongNumArgs(ref cmd, ref req) =>
                write!(f, "command '{}' requires {} args", cmd, req),
            Error::UnknownNick(ref nick) =>
                write!(f, "{} '{}'", self.description(), nick),
        }
    }
}
