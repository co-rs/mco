use std::any::Any;
use std::cell::UnsafeCell;
use std::collections::HashMap;
use std::hash::Hash;
use std::sync::Arc;
use std::sync::atomic::Ordering;
use crate::std::time::time::Time;
use crate::std::errors::{Error, Result};
use crate::std::lazy::sync::Lazy;
use crate::std::map::SyncHashMap;
use crate::std::sync::{AtomicOption, Mutex, Receiver, Sender, Wrapped};

/// A Context carries a deadline, a cancellation signal, and other values across
/// API boundaries.
///
/// Context's methods may be called by multiple goroutines simultaneously.
pub trait Context {
    fn deadline(&self) -> (Time, bool);
    fn done(&self) -> Option<&Receiver<()>>;
    fn err(&self) -> Option<Error>;
}

pub trait Canceler {
    fn cancel(&self, err: Option<Error>);
    fn done(&self) -> Option<&Receiver<()>>;
}

/// CLOSE_CHAN is a reusable closed channel.
static CLOSE_RECV: Lazy<Receiver<()>> = Lazy::new(|| {
    let (s, r) = chan!();
    r
});

pub struct CancelCtx {
    context: Option<Box<dyn Context>>,
    send: Sender<()>,
    done: AtomicOption<Receiver<()>>,
    children: SyncHashMap<String, Box<dyn Canceler>>,
    err: AtomicOption<Error>,
}

unsafe impl Send for CancelCtx {}

unsafe impl Sync for CancelCtx {}


impl_wrapper!(Receiver<()>);
impl_wrapper!(Error);

impl CancelCtx {
    pub fn new_arc(parent: Option<Box<dyn Context>>) -> Arc<Self> {
        Arc::new(Self::new(parent))
    }

    pub fn new(parent: Option<Box<dyn Context>>) -> Self {
        let (s, r) = chan!();
        CancelCtx {
            context: parent,
            send: s,
            done: AtomicOption::some(r),
            children: SyncHashMap::new(),
            err: AtomicOption::none(),
        }
    }
}

impl Canceler for CancelCtx {
    fn cancel(&self, err: Option<Error>) {
        if err.is_none() {
            panic!("context: internal error: missing cancel error")
        }
        if self.err.is_some() {
            return;// already canceled
        }
        self.err.swap(err.clone().unwrap(), Ordering::SeqCst);
        if let Some(v) = self.done.get() {
            self.send.send(());
        } else {
            self.done.swap(CLOSE_RECV.clone(), Ordering::SeqCst);
        }
        for (_, mut v) in self.children.iter_mut() {
            v.cancel(err.clone());
        }
        self.children.clear();
    }

    fn done(&self) -> Option<&Receiver<()>> {
        self.done.get()
    }
}


pub struct TimerCtx {}