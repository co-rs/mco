use std::cell::RefCell;
use crate::std::errors::Error;
use crate::std::sync::channel;
use crate::std::errors::Result;

/// will spawn an thread to doing and return value by channel
pub fn spawn_blocking<F, T>(f: F) -> Result<T>
    where
        F: FnOnce() -> T,
        F: Send + 'static,
        T: Send + 'static,
{
    let (s, r) = channel::<Result<T>>();
    std::thread::spawn(move || {
        let mut finish = RefCell::new(false);
        defer!(||{
           if finish.borrow().eq(&false){
                s.send(Err(Error::from("spawn_blocking panic!")));
            }
        });
        s.send(Ok(f()));
        *finish.borrow_mut() = true;
    });
    return r.recv().unwrap();
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