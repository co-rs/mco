use std::io;
use std::io::ErrorKind;

use crate::cancel::CancelIo;
use crate::std::sync::Mutex;
// use scheduler::get_scheduler;
use super::EventData;

pub struct CancelIoData {
    ev_data: *mut EventData,
}

unsafe impl Send for CancelIoData {}

impl CancelIoData {
    pub fn new(ev_data: &EventData) -> Self {
        CancelIoData {
            ev_data: ev_data as *const _ as *mut _,
        }
    }

    fn cancel(&self) -> Result<(), io::Error> {
        use windows_sys::Win32::System::IO::CancelIoEx;

        let ev = unsafe { &mut *self.ev_data };
        let handle = ev.handle;
        let overlapped = ev.get_overlapped();
        let ret = unsafe { CancelIoEx(handle, overlapped) };
        if ret == 0 {
            let err = io::Error::last_os_error();
            error!("cancel err={:?}", err);
            // ev.co.take().map(|co| get_scheduler().schedule(co));
            Err(err)
        } else {
            Ok(())
        }
    }
}

// windows must use Mutex to protect it's data
// because it will not use the AtomicOption<CoroutineImpl> as a gate keeper
pub struct CancelIoImpl(Mutex<Option<CancelIoData>>);

impl CancelIo for CancelIoImpl {
    type Data = CancelIoData;

    fn new() -> Self {
        CancelIoImpl(Mutex::new(None))
    }

    fn set(&self, data: CancelIoData) {
        *self.0.lock().expect("failed to get CancelIo lock") = Some(data);
    }

    fn clear(&self) {
        *self.0.lock().expect("failed to get CancelIo lock") = None;
    }

    fn cancel(&self) -> Result<(), std::io::Error> {
        match self.0.lock() {
            Ok(mut v) => {
                v.take().map(|d| match d.cancel() {
                    Ok(_) => {
                        return Ok(());
                    }
                    Err(_e) => {
                        return Err("failed to get CancelIo lock");
                    }
                });
                return Ok(());
            }
            Err(e) => {
                return Err(std::io::Error::new(ErrorKind::Other, e.to_string()));
            }
        }
    }
}
