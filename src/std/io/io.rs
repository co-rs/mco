use std::io::{Read, Write};
use crate::std::errors::{Error, Result};
use crate::std::lazy::sync::Lazy;


// ErrShortWrite means that a write accepted fewer bytes than requested
// but failed to return an explicit error.
pub static ErrShortWrite: Lazy<Error> = Lazy::new(|| Error::from("short write"));

// errInvalidWrite means that a write returned an impossible count.
pub static errInvalidWrite: Lazy<Error> = Lazy::new(|| Error::from("invalid write result"));

// ErrShortBuffer means that a read required a longer buffer than was provided.
pub static ErrShortBuffer: Lazy<Error> = Lazy::new(|| Error::from("short buffer"));

// EOF is the error returned by Read when no more input is available.
// (Read must return EOF itself, not an error wrapping EOF,
// because callers will test for EOF using ==.)
// Functions should return EOF only to signal a graceful end of input.
// If the EOF occurs unexpectedly in a structured data stream,
// the appropriate error is either ErrUnexpectedEOF or some other error
// giving more detail.
pub static EOF: Lazy<Error> = Lazy::new(|| Error::from("EOF"));

// ErrUnexpectedEOF means that EOF was encountered in the
// middle of reading a fixed-size block or data structure.
pub static ErrUnexpectedEOF: Lazy<Error> = Lazy::new(|| Error::from("unexpected EOF"));

// ErrNoProgress is returned by some clients of an Reader when
// many calls to Read have failed to return any data or error,
// usually the sign of a broken Reader implementation.
pub static ErrNoProgress: Lazy<Error> = Lazy::new(|| Error::from("multiple Read calls return no data or error"));

// Closer is the interface that wraps the basic Close method.
//
// The behavior of Close after the first call is undefined.
// Specific implementations may document their own behavior.
pub trait Closer {
    fn close(&mut self) -> Result<()>;
}

pub trait ReadCloser: Read + Closer {}

pub trait WriteCloser: Write + Closer {}

struct NopCloser<R> where R: Read {
    pub reader: R,
}

impl<R: std::io::Read> Closer for NopCloser<R> {
    fn close(&mut self) -> crate::std::errors::Result<()> {
        Ok(())
    }
}

/// ReadAll reads from r until an error or EOF and returns the data it read.
/// A successful call returns err == nil, not err == EOF. Because ReadAll is
/// defined to read from src until EOF, it does not treat an EOF from Read
/// as an error to be reported.
pub fn read_all<R: Read>(mut r: R) -> Result<Vec<u8>> {
    let mut b = Vec::with_capacity(512);
    r.read_to_end(&mut b)?;
    return Ok(b);
}