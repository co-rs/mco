// re-export coroutine interface
pub use crate::cancel::trigger_cancel_panic;
pub use crate::coroutine_impl::{
    current, try_current, is_coroutine, park, park_timeout, spawn, go, Builder, Coroutine,
};
pub use crate::join::JoinHandle;
pub use crate::park::ParkError;
pub use crate::scoped::scope;
pub use crate::sleep::sleep;
pub use crate::yield_now::yield_now;

pub trait Spawn {
    /// spawn a new coroutine
    fn spawn<F, T>(self, f: F) -> JoinHandle<T>
        where
            F: FnOnce() -> T + Send + 'static,
            T: Send + 'static;
}

impl Spawn for i32 {
    fn spawn<F, T>(self, f: F) -> JoinHandle<T> where F: FnOnce() -> T + Send + 'static, T: Send + 'static {
        Builder::new().stack_size(self as usize).spawn(f)
    }
}

impl Spawn for &str {
    fn spawn<F, T>(self, f: F) -> JoinHandle<T> where F: FnOnce() -> T + Send + 'static, T: Send + 'static {
        Builder::new().name(self.to_string()).spawn(f)
    }
}

impl Spawn for (&str, i32) {
    fn spawn<F, T>(self, f: F) -> JoinHandle<T> where F: FnOnce() -> T + Send + 'static, T: Send + 'static {
        Builder::new().name(self.0.to_string()).stack_size(self.1 as usize).spawn(f)
    }
}

impl Spawn for (String, i32) {
    fn spawn<F, T>(self, f: F) -> JoinHandle<T> where F: FnOnce() -> T + Send + 'static, T: Send + 'static {
        Builder::new().name(self.0).stack_size(self.1 as usize).spawn(f)
    }
}

impl Spawn for String {
    fn spawn<F, T>(self, f: F) -> JoinHandle<T> where F: FnOnce() -> T + Send + 'static, T: Send + 'static {
        Builder::new().name(self).spawn(f)
    }
}

impl Spawn for &String {
    fn spawn<F, T>(self, f: F) -> JoinHandle<T> where F: FnOnce() -> T + Send + 'static, T: Send + 'static {
        Builder::new().name(self.to_owned()).spawn(f)
    }
}

impl Spawn for Builder {
    fn spawn<F, T>(self, f: F) -> JoinHandle<T> where F: FnOnce() -> T + Send + 'static, T: Send + 'static {
        self.spawn(f)
    }
}