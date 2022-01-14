use std::fmt::{self, Debug, Display};
use std::io::ErrorKind::UnexpectedEof;
use crate::std::io::io;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Error {
    pub inner: String,
}

impl Error {
    pub fn error(&self) -> String {
        self.inner.clone()
    }

    pub fn warp<E>(e: E, info: &str) -> Self where E: std::fmt::Display {
        Self {
            inner: format!("{}{}", info, e)
        }
    }
}

/// warp errors
#[macro_export]
macro_rules! err_warp {
     ($($arg:tt)*) => {{
         $crate::std::errors::Error{
             inner: format!($($arg)*)
         }
     }}
}

///new error
#[inline]
pub fn new(text: String) -> Error {
    Error {
        inner: text
    }
}

pub trait FromError<T>: Sized {
    fn from_err(_: T) -> Error;
}

impl ToString for Error {
    fn to_string(&self) -> String {
        self.inner.clone()
    }
}

impl From<std::io::Error> for Error {
    #[inline]
    fn from(err: std::io::Error) -> Self {
        if err.kind().eq(&UnexpectedEof) {
            return io::EOF.clone();
        }
        if err.kind().eq(&std::io::ErrorKind::UnexpectedEof) {
            return io::ErrUnexpectedEOF.clone();
        }
        new(err.to_string())
    }
}

impl From<&str> for Error {
    fn from(arg: &str) -> Self {
        return new(arg.to_string());
    }
}

impl From<std::string::String> for Error {
    fn from(arg: String) -> Self {
        return new(arg);
    }
}

impl From<&dyn std::error::Error> for Error {
    fn from(arg: &dyn std::error::Error) -> Self {
        return new(arg.to_string());
    }
}

impl From<Box<dyn std::error::Error>> for Error {
    fn from(arg: Box<dyn std::error::Error>) -> Self {
        return new(arg.to_string());
    }
}

impl From<&Box<dyn std::error::Error>> for Error {
    fn from(arg: &Box<dyn std::error::Error>) -> Self {
        return new(arg.to_string());
    }
}

impl From<time::error::InvalidFormatDescription> for Error {
    fn from(arg: time::error::InvalidFormatDescription) -> Self {
        return new(arg.to_string());
    }
}

impl From<time::error::Parse> for Error {
    fn from(arg: time::error::Parse) -> Self {
        return new(arg.to_string());
    }
}