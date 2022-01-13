use crate::std::errors::Result;

// Closer is the interface that wraps the basic Close method.
//
// The behavior of Close after the first call is undefined.
// Specific implementations may document their own behavior.
pub trait Closer {
    fn close(&mut self) -> Result<()>;
}