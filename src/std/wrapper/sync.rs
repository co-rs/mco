//! This module contains a type that can make `Send + !Sync` types `Sync` by
//! disallowing all immutable access to the value.
//!
//! A similar primitive is provided in the `sync_wrapper` crate.

use std::ops::{Deref, DerefMut};

pub struct SyncWrapper<T> {
    inner: T,
}

// safety: The SyncWrapper being send allows you to send the inner value across
// thread boundaries.
unsafe impl<T: Send> Send for SyncWrapper<T> {}

// safety: An immutable reference to a SyncWrapper is useless, so moving such an
// immutable reference across threads is safe.
unsafe impl<T> Sync for SyncWrapper<T> {}

impl<T> SyncWrapper<T> {
    pub fn new(value: T) -> Self {
        Self { inner: value }
    }

    pub fn into_inner(self) -> T {
        self.inner
    }
}

impl<T> Deref for SyncWrapper<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> DerefMut for SyncWrapper<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
