//! compatible with std::sync::mpsc except for both thread and coroutine
//! please ref the doc from std::sync::mpsc

pub use crate::std::sync::mpmc::channel;
pub use crate::std::sync::mpmc::channel_buf;
pub use crate::std::sync::mpmc::unbounded;
pub use crate::std::sync::mpmc::bounded;
pub use crate::std::sync::mpmc::{Sender, Receiver};
