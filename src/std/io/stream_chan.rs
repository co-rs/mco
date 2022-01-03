use std::sync::mpsc::RecvError;
use crate::std::errors::Error;
use crate::std::io::{Stream, TryStream};
use crate::std::sync::mpsc::{Receiver, Sender};

/// ChanStream,based on mpsc channel.when send Err data stop next
pub struct ChanStream<T> {
    pub recv: Receiver<Result<T, Error>>,
    pub send: Sender<Result<T, Error>>,
}

impl<T> Stream for ChanStream<T> {
    type Item = Result<T, crate::std::errors::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.recv.recv() {
            Ok(v) => {
                if v.is_err() {
                    return None;
                }
                Some(v)
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
    use crate::std::sync::mpsc::channel;
    use crate::std::errors::Error;
    use std::ops::ControlFlow;

    #[test]
    fn test_foreach() {
        let (s, r) = channel::<Result<i32, Error>>();
        s.send(Ok(1));
        s.send(Err(Error::from("done")));
        let mut c = ChanStream {
            recv: r,
            send: s,
        };
        for item in c {
            println!("{:?}", item);
        }
    }
}

