use std::sync::mpsc::RecvError;
use crate::std::errors::Error;
use crate::std::io::{Stream, TryStream};
use crate::std::sync::channel::{Receiver, Sender, unbounded};


/// ChanStream,based on mpsc channel.when send Err data stop next
pub struct ChanStream<T, E> {
    pub recv: Receiver<Option<Result<T, E>>>,
    pub send: Sender<Option<Result<T, E>>>,
}

impl<T, E> ChanStream<T, E> {
    pub fn new<'s, F>(f: F) -> Self
        where F: FnOnce(Sender<Option<Result<T, E>>>) -> Result<(), E>,
              E: From<&'s str> {
        let (s, r) = unbounded();
        let result = f(s.clone());
        //send none, make sure work is done
        if let Err(e) = result {
            s.send(Some(Err(e)));
        }
        s.send(None);
        Self {
            recv: r,
            send: s,
        }
    }
}

impl<T, E> Stream for ChanStream<T, E> {
    type Item = Result<T, E>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.recv.recv() {
            Ok(v) => {
                return match v {
                    None => {
                        None
                    }
                    Some(v) => {
                        Some(v)
                    }
                };
            }
            Err(e) => {
                None
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::std::io::{ChanStream, TryStream};
    use crate::std::sync::channel::channel;
    use crate::std::errors::Error;
    use std::ops::ControlFlow;

    #[test]
    fn test_foreach() {
        let (s, r) = channel::<Option<Result<i32, Error>>>();
        s.send(Some(Ok(1)));
        s.send(None);
        let mut c = ChanStream {
            recv: r,
            send: s,
        };
        for item in c {
            println!("{:?}", item);
        }
    }

    #[test]
    fn test_map() {
        let (s, r) = channel::<Option<Result<i32, Error>>>();
        s.send(Some(Ok(1)));
        s.send(None);
        let mut c = ChanStream {
            recv: r,
            send: s,
        };
        let data = c.map(|v| {
            match v {
                Ok(v) => { return Some(v); }
                Err(_) => {}
            }
            None
        }).collect::<Vec<Option<i32>>>();
        println!("{:?}", data);
    }
}

