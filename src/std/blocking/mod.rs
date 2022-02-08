use std::cell::RefCell;
use std::panic::set_hook;
use crate::std::errors::Error;
use crate::std::sync::channel;
use crate::std::errors::Result;


/// will spawn a thread to doing and return value by channel
/// for example:
/// ```rust
///     let v = cogo::spawn_blocking!(|| {
///         //do something Heavy CPU arithmetic and blocking APIS
///         return 1;
///     });
///     assert_eq!(v.unwrap(), 1);
/// ```
#[macro_export]
macro_rules! spawn_blocking {
    ($task:expr) => {
        if true{
            $crate::std::blocking::spawn_blocking($task)
        }else{
            Ok($task())
        }
    };
}


/// will spawn a thread to doing and return value by channel
/// for example:
/// ```rust
///     let v = cogo::std::blocking::spawn_blocking(|| {
///         //do something Heavy CPU arithmetic and blocking APIS
///         return 1;
///     });
///     assert_eq!(v.unwrap(), 1);
/// ```
pub fn spawn_blocking<F, T>(f: F) -> Result<T>
    where
        F: FnOnce() -> T,
        F: Send + 'static,
        T: Send + 'static,
{
    let (s, r) = channel::<Result<T>>();
    std::thread::spawn(move || {
        let send_e = s.clone();
        set_hook(Box::new(move |panic_info|{
            let e= err!("{}",panic_info.payload().downcast_ref::<&str>().unwrap_or(&"spawn_blocking panic!"));
            send_e.send(Err(e));
        }));
        s.send(Ok(f()));
    });
    return r.recv()?;
}


#[cfg(test)]
mod test {
    use crate::std::blocking::spawn_blocking;

    #[test]
    fn test_spawn_blocking() {
        let v = spawn_blocking(|| {
            return 1;
        });
        assert_eq!(v.unwrap(), 1);
    }

    #[test]
    fn test_spawn_panic() {
        let v = spawn_blocking(|| {
            panic!("e");
            return 1;
        });
        assert_eq!(v.is_err(), true);
    }
}