use std::sync::atomic::{AtomicU32, Ordering};
use crate::defer;
use crate::std::sync::Mutex;

/// Once is an object that will perform exactly one action.
///
/// A Once must not be copied after first use.
pub struct Once {
    done: AtomicU32,
}

impl Once {
    pub fn new() -> Self {
        Self {
            done: Default::default(),
        }
    }
    /// Do calls the function f if and only if Do is being called for the
    /// first time for this instance of Once. In other words, given
    /// 	var once Once
    /// if once.Do(f) is called multiple times, only the first call will invoke f,
    /// even if f has a different value in each invocation. A new instance of
    /// Once is required for each function to execute.
    ///
    /// Do is intended for initialization that must be run exactly once. Since f
    /// is niladic, it may be necessary to use a function literal to capture the
    /// arguments to a function to be invoked by Do:
    /// 	config.once.do(|| { config.init(filename) })
    ///
    /// Because no call to Do returns until the one call to f returns, if f causes
    /// Do to be called, it will deadlock.
    ///
    /// If f panics, Do considers it to have returned; future calls of Do return
    /// without calling f.
    pub fn r#do<F>(&self, f: F) where F: FnMut() {
        if self.done.load(Ordering::SeqCst) == 0 {
            self.do_slow(f);
        }
    }

    fn do_slow<F>(&self, mut f: F) where F: FnMut() {
        if self.done.load(Ordering::SeqCst) == 0 {
            self.done.store(1,Ordering::SeqCst);
            f();
        }
    }
}

#[cfg(test)]
mod test {
    use std::cell::UnsafeCell;
    use std::panic::catch_unwind;
    use std::sync::Arc;
    use crate::{chan, defer};
    use crate::std::sync::mpmc::Sender;
    use crate::std::sync::Once;

    pub struct One {
        pub inner: UnsafeCell<i32>,
    }

    unsafe impl Send for One {}

    unsafe impl Sync for One {}

    impl One {
        pub fn increment(&self) {
            *(unsafe { &mut *self.inner.get() }) += 1;
        }
        pub fn value(&self) -> i32 {
            unsafe {
                *self.inner.get()
            }
        }
    }


    unsafe fn run(once: Arc<Once>, o: Arc<One>, s: Arc<Sender<bool>>) {
        once.r#do(|| {
            o.increment();
        });
        if o.value() != 1 {
            panic!(format!("once failed inside run: {} is not 1", o.value()));
        }
        s.send(true);
    }

    #[test]
    fn test_once() {
        let one = Arc::new(One { inner: UnsafeCell::new(0) });
        let once = Arc::new(Once::new());
        let (s, r) = chan!();
        let sender = Arc::new(s);
        let n = 10;
        for i in 0..n {
            let oc = once.clone();
            let s = sender.clone();
            let o = one.clone();
            go!(move ||{unsafe {run(oc,o,s);}});
        }
        for i in 0..n {
            r.recv();
        }
        if one.value() != 1 {
            panic!(format!("once failed outside run: {} is not 1", one.value()));
        }
    }

    #[test]
    fn test_once_panic() {
        let once = Once::new();
        catch_unwind(|| {
            once.r#do(|| {
                panic!("failed");
            });
        });
        once.r#do(|| {
            panic!("Once.Do called twice");
        });
    }
}