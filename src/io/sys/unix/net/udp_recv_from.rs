use std::net::SocketAddr;
use std::sync::atomic::Ordering;
use std::time::Duration;
use std::{self, io};

use super::super::{co_io_result, IoData};
use crate::coroutine_impl::{co_get_handle, CoroutineImpl, EventSource};
use crate::io::AsIoData;
use crate::net::UdpSocket;
use crate::scheduler::get_scheduler;
use crate::yield_now::yield_with;

pub struct UdpRecvFrom<'a> {
    io_data: &'a IoData,
    buf: &'a mut [u8],
    socket: &'a std::net::UdpSocket,
    timeout: Option<Duration>,
}

impl<'a> UdpRecvFrom<'a> {
    pub fn new(socket: &'a UdpSocket, buf: &'a mut [u8]) -> Self {
        UdpRecvFrom {
            io_data: socket.as_io_data(),
            buf,
            socket: socket.inner(),
            timeout: socket.read_timeout().unwrap(),
        }
    }

    pub fn done(&mut self) -> io::Result<(usize, SocketAddr)> {
        loop {
            co_io_result()?;

            // clear the io_flag
            self.io_data.io_flag.store(false, Ordering::Relaxed);

            match self.socket.recv_from(self.buf) {
                Ok(n) => return Ok(n),
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

impl<'a> EventSource for UdpRecvFrom<'a> {
    fn subscribe(&mut self, co: CoroutineImpl) {
        let handle = co_get_handle(&co);
        let cancel = handle.get_cancel();
        let io_data = (*self.io_data).clone();

        if let Some(dur) = self.timeout {
            get_scheduler()
                .get_selector()
                .add_io_timer(self.io_data, dur);
        }
        self.io_data.co.swap(co);

        // there is event, re-run the coroutine
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
