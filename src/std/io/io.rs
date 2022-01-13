use std::io::{Read, Write};
use crate::std::errors::Result;

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