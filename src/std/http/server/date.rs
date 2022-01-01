use std::cell::UnsafeCell;
use std::fmt::{self, Write};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

use bytes::BytesMut;
use lazy_static::lazy_static;

// Sat, 01 Jan 2022 16:01:09 GMT
const DATE_VALUE_LENGTH_HDR: usize = 39;
const DATE_VALUE_DEFAULT: [u8; DATE_VALUE_LENGTH_HDR] = [
    b'd', b'a', b't', b'e', b':', b' ', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0',
    b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0',
    b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'\r', b'\n', b'\r', b'\n',
];

lazy_static! {
    static ref CURRENT_DATE: Arc<DataWrap> = {
        let date = Arc::new(DataWrap(UnsafeCell::new(Date::new())));
        let date_clone = date.clone();
        spawn_update(date_clone);
        date
    };
}

fn spawn_update(date_clone:Arc<DataWrap>){
     crate::go!(move || loop {
            crate::coroutine::sleep(std::time::Duration::from_millis(500));
            unsafe { &mut *(date_clone.0).get() }.update();
     });
}


struct DataWrap(UnsafeCell<Date>);
unsafe impl Sync for DataWrap {}

#[doc(hidden)]
pub fn set_date(dst: &mut BytesMut) {
    let date = unsafe { &*CURRENT_DATE.0.get() };
    dst.extend_from_slice(date.as_bytes());
}


struct Date {
    inner:String,
}

impl Date {
    fn new() -> Date {
        let mut s=Date{
            inner: String::from_utf8(DATE_VALUE_DEFAULT.to_vec()).unwrap_or_default(),
        };
        s.update();
        s
    }

    #[inline]
    fn as_bytes(&self) -> &[u8] {
        self.inner.as_bytes()
    }

    fn update(&mut self) {
        let mut bytes = DATE_VALUE_DEFAULT;
        let dt = httpdate::HttpDate::from(std::time::SystemTime::now()).to_string();
        bytes[6..35].copy_from_slice(dt.as_ref());
        self.inner = String::from_utf8(bytes.to_vec()).unwrap_or_default().trim().to_string();
    }
}

impl fmt::Write for Date {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.inner = s.to_string();
        Ok(())
    }
}
