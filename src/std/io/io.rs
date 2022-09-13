use crate::std::errors::{Error, Result};
use crate::std::lazy::sync::Lazy;
use std::io::{Read, Write};

// ERR_SHORT_WRITE means that a write accepted fewer bytes than requested
// but failed to return an explicit error.
pub static ERR_SHORT_WRITE: Lazy<Error> = Lazy::new(|| err!("short write"));

// ERR_INVALID_WRITE means that a write returned an impossible count.
pub static ERR_INVALID_WRITE: Lazy<Error> = Lazy::new(|| err!("invalid write result"));

// ERR_SHORT_BUFFER means that a read required a longer buffer than was provided.
pub static ERR_SHORT_BUFFER: Lazy<Error> = Lazy::new(|| err!("short buffer"));

// EOF is the error returned by Read when no more input is available.
// (Read must return EOF itself, not an error wrapping EOF,
// because callers will test for EOF using ==.)
// Functions should return EOF only to signal a graceful end of input.
// If the EOF occurs unexpectedly in a structured data stream,
// the appropriate error is either ERR_UNEXPECTED_EOF or some other error
// giving more detail.
pub static EOF: Lazy<Error> = Lazy::new(|| err!("EOF"));

// ERR_UNEXPECTED_EOF means that EOF was encountered in the
// middle of reading a fixed-size block or data structure.
pub static ERR_UNEXPECTED_EOF: Lazy<Error> = Lazy::new(|| err!("unexpected EOF"));

// ERR_NO_PROGRESS is returned by some clients of an Reader when
// many calls to Read have failed to return any data or error,
// usually the sign of a broken Reader implementation.
pub static ERR_NO_PROGRESS: Lazy<Error> =
    Lazy::new(|| err!("multiple Read calls return no data or error"));

// Closer is the interface that wraps the basic Close method.
//
// The behavior of Close after the first call is undefined.
// Specific implementations may document their own behavior.
pub trait Closer {
    fn close(&mut self) -> Result<()>;
}

pub trait ReadCloser: Read + Closer {}

pub trait WriteCloser: Write + Closer {}

/// ReadAll reads from r until an error or EOF and returns the data it read.
/// A successful call returns err == nil, not err == EOF. Because ReadAll is
/// defined to read from src until EOF, it does not treat an EOF from Read
/// as an error to be reported.
pub fn read_all<R: Read>(mut r: R) -> Result<Vec<u8>> {
    let mut b = Vec::with_capacity(512);
    r.read_to_end(&mut b)?;
    return Ok(b);
}
