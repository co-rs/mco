//! Multi-producer multi-consumer channels for message passing.
//!
//! This crate is an alternative to [`std::sync::mpsc`] with more features and better performance.
//!
//! # Hello, world!
//!
//! ```
//! use cogo::std::channel::unbounded;
//!
//! // Create a channel of unbounded capacity.
//! let (s, r) = unbounded();
//!
//! // Send a message into the channel.
//! s.send("Hello, world!").unwrap();
//!
//! // Receive the message from the channel.
//! assert_eq!(r.recv(), Ok("Hello, world!"));
//! ```
//!
//! # Channel types
//!
//! Channels can be created using two functions:
//!
//! * [`bounded`] creates a channel of bounded capacity, i.e. there is a limit to how many messages
//!   it can hold at a time.
//!
//! * [`unbounded`] creates a channel of unbounded capacity, i.e. it can hold any number of
//!   messages at a time.
//!
//! Both functions return a [`Sender`] and a [`Receiver`], which represent the two opposite sides
//! of a channel.
//!
//! Creating a bounded channel:
//!
//! ```
//! use cogo::std::channel::bounded;
//!
//! // Create a channel that can hold at most 5 messages at a time.
//! let (s, r) = bounded(5);
//!
//! // Can send only 5 messages without blocking.
//! for i in 0..5 {
//!     s.send(i).unwrap();
//! }
//!
//! // Another call to `send` would block because the channel is full.
//! // s.send(5).unwrap();
//! ```
//!
//! Creating an unbounded channel:
//!
//! ```
//! use cogo::std::channel::unbounded;
//!
//! // Create an unbounded channel.
//! let (s, r) = unbounded();
//!
//! // Can send any number of messages into the channel without blocking.
//! for i in 0..1000 {
//!     s.send(i).unwrap();
//! }
//! ```
//!
//! A special case is zero-capacity channel, which cannot hold any messages. Instead, send and
//! receive operations must appear at the same time in order to pair up and pass the message over:
//!
//! ```
//! use std::thread;
//! use cogo::std::channel::bounded;
//!
//! // Create a zero-capacity channel.
//! let (s, r) = bounded(0);
//!
//! // Sending blocks until a receive operation appears on the other side.
//! thread::spawn(move || s.send("Hi!").unwrap());
//!
//! // Receiving blocks until a send operation appears on the other side.
//! assert_eq!(r.recv(), Ok("Hi!"));
//! ```
//!
//! # Sharing channels
//!
//! Senders and receivers can be cloned and sent to other threads:
//!
//! ```
//! use std::thread;
//! use cogo::std::channel::bounded;
//!
//! let (s1, r1) = bounded(0);
//! let (s2, r2) = (s1.clone(), r1.clone());
//!
//! // Spawn a thread that receives a message and then sends one.
//! thread::spawn(move || {
//!     r2.recv().unwrap();
//!     s2.send(2).unwrap();
//! });
//!
//! // Send a message and then receive one.
//! s1.send(1).unwrap();
//! r1.recv().unwrap();
//! ```
//!
//! Note that cloning only creates a new handle to the same sending or receiving side. It does not
//! create a separate stream of messages in any way:
//!
//! ```
//! use cogo::std::channel::unbounded;
//!
//! let (s1, r1) = unbounded();
//! let (s2, r2) = (s1.clone(), r1.clone());
//! let (s3, r3) = (s2.clone(), r2.clone());
//!
//! s1.send(10).unwrap();
//! s2.send(20).unwrap();
//! s3.send(30).unwrap();
//!
//! assert_eq!(r3.recv(), Ok(10));
//! assert_eq!(r1.recv(), Ok(20));
//! assert_eq!(r2.recv(), Ok(30));
//! ```
//!
//! It's also possible to share senders and receivers by reference:
//!
//! ```
//! use cogo::std::channel::bounded;
//! use crossbeam_utils::thread::scope;
//!
//! let (s, r) = bounded(0);
//!
//! scope(|scope| {
//!     // Spawn a thread that receives a message and then sends one.
//!     scope.spawn(|_| {
//!         r.recv().unwrap();
//!         s.send(2).unwrap();
//!     });
//!
//!     // Send a message and then receive one.
//!     s.send(1).unwrap();
//!     r.recv().unwrap();
//! }).unwrap();
//! ```
//!
//! # Disconnection
//!
//! When all senders or all receivers associated with a channel get dropped, the channel becomes
//! disconnected. No more messages can be sent, but any remaining messages can still be received.
//! Send and receive operations on a disconnected channel never block.
//!
//! ```
//! use cogo::std::channel::{unbounded, RecvError};
//!
//! let (s, r) = unbounded();
//! s.send(1).unwrap();
//! s.send(2).unwrap();
//! s.send(3).unwrap();
//!
//! // The only sender is dropped, disconnecting the channel.
//! drop(s);
//!
//! // The remaining messages can be received.
//! assert_eq!(r.recv(), Ok(1));
//! assert_eq!(r.recv(), Ok(2));
//! assert_eq!(r.recv(), Ok(3));
//!
//! // There are no more messages in the channel.
//! assert!(r.is_empty());
//!
//! // Note that calling `r.recv()` does not block.
//! // Instead, `Err(RecvError)` is returned immediately.
//! assert_eq!(r.recv(), Err(RecvError));
//! ```
//!
//! # Blocking operations
//!
//! Send and receive operations come in three flavors:
//!
//! * Non-blocking (returns immediately with success or failure).
//! * Blocking (waits until the operation succeeds or the channel becomes disconnected).
//! * Blocking with a timeout (blocks only for a certain duration of time).
//!
//! A simple example showing the difference between non-blocking and blocking operations:
//!
//! ```
//! use cogo::std::channel::{bounded, RecvError, TryRecvError};
//!
//! let (s, r) = bounded(1);
//!
//! // Send a message into the channel.
//! s.send("foo").unwrap();
//!
//! // This call would block because the channel is full.
//! // s.send("bar").unwrap();
//!
//! // Receive the message.
//! assert_eq!(r.recv(), Ok("foo"));
//!
//! // This call would block because the channel is empty.
//! // r.recv();
//!
//! // Try receiving a message without blocking.
//! assert_eq!(r.try_recv(), Err(TryRecvError::Empty));
//!
//! // Disconnect the channel.
//! drop(s);
//!
//! // This call doesn't block because the channel is now disconnected.
//! assert_eq!(r.recv(), Err(RecvError));
//! ```
//!
//! # Iteration
//!
//! Receivers can be used as iterators. For example, method [`iter`] creates an iterator that
//! receives messages until the channel becomes empty and disconnected. Note that iteration may
//! block waiting for next message to arrive.
//!
//! ```
//! use std::thread;
//! use cogo::std::channel::unbounded;
//!
//! let (s, r) = unbounded();
//!
//! thread::spawn(move || {
//!     s.send(1).unwrap();
//!     s.send(2).unwrap();
//!     s.send(3).unwrap();
//!     drop(s); // Disconnect the channel.
//! });
//!
//! // Collect all messages from the channel.
//! // Note that the call to `collect` blocks until the sender is dropped.
//! let v: Vec<_> = r.iter().collect();
//!
//! assert_eq!(v, [1, 2, 3]);
//! ```
//!
//! A non-blocking iterator can be created using [`try_iter`], which receives all available
//! messages without blocking:
//!
//! ```
//! use cogo::std::channel::unbounded;
//!
//! let (s, r) = unbounded();
//! s.send(1).unwrap();
//! s.send(2).unwrap();
//! s.send(3).unwrap();
//! // No need to drop the sender.
//!
//! // Receive all messages currently in the channel.
//! let v: Vec<_> = r.try_iter().collect();
//!
//! assert_eq!(v, [1, 2, 3]);
//! ```
//!
//! # Selection
//!
//! The [`cogo::select!`] macro allows you to define a set of channel operations, wait until any one of
//! them becomes ready, and finally execute it. If multiple operations are ready at the same time,
//! a random one among them is selected.
//!
//! It is also possible to define a `default` case that gets executed if none of the operations are
//! ready, either right away or for a certain duration of time.
//!
//! An operation is considered to be ready if it doesn't have to block. Note that it is ready even
//! when it will simply return an error because the channel is disconnected.
//!
#![doc(test(
no_crate_inject,
attr(
deny(warnings, rust_2018_idioms),
allow(dead_code, unused_assignments, unused_variables)
)
))]
#![warn(
missing_docs,
missing_debug_implementations,
rust_2018_idioms,
unreachable_pub
)]

mod channel;
mod context;
mod counter;
mod err;
mod flavors;
mod select;
mod utils;
mod waker;

/// Crate internals used by the `cogo::select!` macro.
#[doc(hidden)]
pub mod internal {
    pub use crate::std::channel::select::SelectHandle;
    pub use crate::std::channel::select::{select, select_timeout, try_select};
}

pub use crate::std::channel::channel::{tick};
pub use crate::std::channel::channel::{bounded, unbounded};
pub use crate::std::channel::channel::{IntoIter, Iter, TryIter};
pub use crate::std::channel::channel::{Receiver, Sender};

pub use crate::std::channel::select::{Select, SelectedOperation};

pub use crate::std::channel::err::{ReadyTimeoutError, SelectTimeoutError, TryReadyError, TrySelectError};
pub use crate::std::channel::err::{RecvError, RecvTimeoutError, TryRecvError};
pub use crate::std::channel::err::{SendError, SendTimeoutError, TrySendError};
