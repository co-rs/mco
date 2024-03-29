use std::io;
use std::os::unix::net::{self, SocketAddr};
use std::sync::atomic::Ordering;

use crate::coroutine_impl::{co_get_handle, CoroutineImpl, EventSource};
use crate::io::sys::{co_io_result, IoData};
use crate::io::{AsIoData, CoIo};
use crate::os::unix::net::{UnixListener, UnixStream};
use crate::yield_now::yield_with;

pub struct UnixListenerAccept<'a> {
    io_data: &'a IoData,
    socket: &'a net::UnixListener,
}

impl<'a> UnixListenerAccept<'a> {
    pub fn new(socket: &'a UnixListener) -> io::Result<Self> {
        Ok(UnixListenerAccept {
            io_data: socket.0.as_io_data(),
            socket: socket.0.inner(),
        })
    }

    pub fn done(&mut self) -> io::Result<(UnixStream, SocketAddr)> {
        loop {
            co_io_result()?;

            // clear the io_flag
            self.io_data.io_flag.store(false, Ordering::Relaxed);

            match self.socket.accept() {
                Ok((s, a)) => {
                    let s = UnixStream::from_coio(CoIo::new(s)?);
                    return Ok((s, a));
                }
                Err(e) => {
                    // raw_os_error is faster than kind
                    let raw_err = e.raw_os_error();
                    if raw_err == Some(libc::EAGAIN) || raw_err == Some(libc::EWOULDBLOCK) {
                        // do nothing here
                    } else {
                        return Err(e);
                    }
                }
            }

            if self.io_data.io_flag.swap(false, Ordering::Relaxed) {
                continue;
            }

            // the result is still WouldBlock, need to try again
            yield_with(self);
        }
    }
}

impl<'a> EventSource for UnixListenerAccept<'a> {
    fn subscribe(&mut self, co: CoroutineImpl) {
        let handle = co_get_handle(&co);
        let cancel = handle.get_cancel();
        let io_data = (*self.io_data).clone();

        // if there is no timer we don't need to call add_io_timer
        self.io_data.co.swap(co);

        // there is event happened
        if io_data.io_flag.load(Ordering::Acquire) {
            return io_data.schedule();
        }

        // register the cancel io data
        cancel.set_io(io_data);
        // re-check the cancel status
        if cancel.is_canceled() {
            let _ = cancel.cancel();
        }
    }
}
