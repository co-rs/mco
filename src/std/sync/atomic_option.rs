use crossbeam_utils::atomic::AtomicCell;
use std::borrow::Borrow;
use std::fmt::{Debug, Formatter};

pub struct AtomicOption<T> {
    inner: AtomicCell<Option<T>>,
}

impl<T: std::fmt::Debug> Debug for AtomicOption<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.get() {
            None => f
                .debug_struct("AtomicOption")
                .field("inner", &Option::<T>::None)
                .finish(),
            Some(s) => f.debug_struct("AtomicOption").field("inner", s).finish(),
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
        unsafe { (&(*self.inner.borrow().as_ptr())).is_none() }
    }

    #[inline]
    pub fn is_some(&self) -> bool {
        unsafe { (&(*self.inner.borrow().as_ptr())).is_some() }
    }

    #[inline]
    pub fn get(&self) -> Option<&T> {
        match unsafe { &(*self.inner.borrow().as_ptr()) } {
            None => None,
            Some(v) => Some(v),
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
