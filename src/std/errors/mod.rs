use std::fmt::{self, Debug, Display};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub struct Error {
    pub inner: Box<dyn std::error::Error>,
}

impl Error {
    pub fn error(&self) -> String {
        self.inner.to_string()
    }
}

impl Clone for Error {
    fn clone(&self) -> Self {
        Error {
            inner: self.to_string().into()
        }
    }
}

impl PartialEq<Self> for Error {
    fn eq(&self, other: &Self) -> bool {
        self.inner.to_string().eq(&other.to_string())
    }
}

impl Eq for Error {}

///new error
pub fn new(text: String) -> Error {
    Error {
        inner: text.into()
    }
}

pub trait FromError<T>: Sized {
    fn from_err(_: T) -> Error;
}

impl ToString for Error {
    fn to_string(&self) -> String {
        self.inner.to_string()
    }
}

impl From<std::io::Error> for Error {
    #[inline]
    fn from(err: std::io::Error) -> Self {
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
        return Self {
            inner: arg
        };
    }
}

impl From<&Box<dyn std::error::Error>> for Error {
    fn from(arg: &Box<dyn std::error::Error>) -> Self {
        return new(arg.to_string());
    }
}
