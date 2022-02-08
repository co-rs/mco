use std::borrow::Borrow;
use std::fmt::{Debug, Formatter};
use std::ptr;
use std::sync::atomic::{AtomicPtr, Ordering};
use std::sync::Arc;
use crossbeam_utils::atomic::AtomicCell;

use generator::Generator;

// heap based wrapper for a type
pub trait Wrapped {
    type Data;
    fn into_raw(self) -> *mut Self::Data;
    unsafe fn from_raw(_: *mut Self::Data) -> Self;
}


#[macro_export]
macro_rules! impl_wrapper {
    ($t:ty) => {
        impl $crate::std::sync::Wrapped for $t {
    type Data = $t;
    fn into_raw(self) -> *mut Self::Data {
        Box::into_raw(Box::new(self)) as _
    }
    unsafe fn from_raw(p: *mut Self::Data) -> $t {
        *Box::from_raw(p as _)
    }
}
    };
}


impl<T> Wrapped for *mut T {
    type Data = T;
    fn into_raw(self) -> *mut T {
        self
    }
    unsafe fn from_raw(p: *mut T) -> *mut T {
        p
    }
}

impl<T> Wrapped for Arc<T> {
    type Data = T;
    fn into_raw(self) -> *mut T {
        Arc::into_raw(self) as *mut _
    }
    unsafe fn from_raw(p: *mut T) -> Arc<T> {
        Arc::from_raw(p)
    }
}

impl<T> Wrapped for Box<T> {
    type Data = T;
    fn into_raw(self) -> *mut T {
        Box::into_raw(self)
    }
    unsafe fn from_raw(p: *mut T) -> Box<T> {
        Box::from_raw(p)
    }
}

impl<'a, A, T> Wrapped for Generator<'a, A, T> {
    type Data = usize;
    fn into_raw(self) -> *mut usize {
        Generator::into_raw(self)
    }
    unsafe fn from_raw(p: *mut usize) -> Self {
        Generator::from_raw(p)
    }
}

pub struct AtomicOption<T> {
    inner: AtomicCell<Option<T>>,
}

impl<T: std::fmt::Debug> Debug for AtomicOption<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.get() {
            None => {
                f.debug_struct("AtomicOption")
                    .field("inner", &Option::<T>::None)
                    .finish()
            }
            Some(s) => {
                f.debug_struct("AtomicOption")
                    .field("inner", s)
                    .finish()
            }
        }
    }
}

unsafe impl<T> Send for AtomicOption<T> {}

unsafe impl<T> Sync for AtomicOption<T> {}

impl<T> AtomicOption<T> {
    pub fn none() -> AtomicOption<T> {
        AtomicOption {
            inner: AtomicCell::new(None),
        }
    }

    pub fn some(t: T) -> AtomicOption<T> {
        AtomicOption {
            inner: AtomicCell::new(Some(t)),
        }
    }

    #[inline]
    pub fn store(&self, t: T) {
        self.inner.store(Some(t))
    }

    #[inline]
    pub fn swap(&self, t: T) -> Option<T> {
        self.inner.swap(Some(t))
    }

    #[inline]
    pub fn take(&self) -> Option<T> {
        self.inner.take()
    }

    #[inline]
    pub fn is_none(&self) -> bool {
        unsafe {
            (&(*self.inner.borrow().as_ptr())).is_none()
        }
    }

    #[inline]
    pub fn is_some(&self) -> bool {
        unsafe {
            (&(*self.inner.borrow().as_ptr())).is_some()
        }
    }

    #[inline]
    pub fn get(&self) -> Option<&T> {
        match unsafe { (&(*self.inner.borrow().as_ptr())) } {
            None => {
                None
            }
            Some(v) => {
                Some(v)
            }
        }
    }
}

impl<T> Default for AtomicOption<T> {
    fn default() -> Self {
        Self::none()
    }
}

impl<T> Drop for AtomicOption<T> {
    fn drop(&mut self) {
        self.inner.store(None)
    }
}
