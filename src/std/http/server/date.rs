use std::cell::UnsafeCell;
use std::fmt::{self, Write};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use bytes::BytesMut;

#[doc(hidden)]
#[inline]
pub fn set_date(dst: &mut BytesMut) {
    let date = httpdate::HttpDate::from(std::time::SystemTime::now()).to_string();
    dst.extend_from_slice(date.as_bytes());
}