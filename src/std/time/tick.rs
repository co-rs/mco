use crate::coroutine::sleep;
use crate::std::errors::Result;
use crate::std::sync::channel::Receiver;
use crate::std::sync::Mutex;
use crate::std::time::time::Time;
use std::ops::Deref;
use std::sync::Arc;
use std::time::Duration;

/// A Ticker holds a channel that delivers ``ticks'' of a clock
/// at intervals.
/// for example:
/// ```
///         use mco::coroutine::sleep;
///         use mco::std::time::tick::Ticker;
///         use std::sync::Arc;
///         use std::time::Duration;
///         use mco::co;
///
///         let mut t = Arc::new(Ticker::new(Duration::from_secs(1)));
///         let tclone = t.clone();
///         co!(move ||{
///              for x in tclone.as_ref() {
///                println!("tick {}", x);
///             }
///         });
///         sleep(Duration::from_secs(3));
///         t.stop();
///
/// ```
pub struct Ticker {
    pub d: Arc<Mutex<Duration>>,
    pub recv: Receiver<Time>,
}

impl Ticker {
    pub fn new_arc(d: Duration) -> Arc<Self> {
        Arc::new(Self::new(d))
    }

    pub fn new(d: Duration) -> Self {
        let d = Arc::new(Mutex::new(d));
        let (s, r) = chan!();
        let ticker = Self { d: d, recv: r };
        let d = ticker.d.clone();
        let tick = move || loop {
            match d.lock() {
                Ok(d) => {
                    if d.is_zero() {
                        break;
                    }
                    sleep(d.deref().clone());
                    let _ = s.send(Time::now());
                }
                Err(_) => {
                    break;
                }
            }
        };
        co!(tick);
        ticker
    }

    /// Stop turns off a ticker. After Stop, no more ticks will be sent.
    /// Stop does not close the channel, to prevent a concurrent goroutine
    /// reading from the channel from seeing an erroneous "tick".
    pub fn stop(&self) -> Result<()> {
        match self.d.lock() {
            Ok(mut d) => {
                *d = Duration::from_secs(0);
                Ok(())
            }
            Err(e) => Err(err!("lock fail: {}", e)),
        }
    }

    /// Reset stops a ticker and resets its period to the specified duration.
    /// The next tick will arrive after the new period elapses.
    pub fn reset(&self, d: Duration) -> Result<()> {
        match self.d.lock() {
            Ok(mut dur) => {
                *dur = d;
                Ok(())
            }
            Err(e) => Err(err!("lock fail: {}", e)),
        }
    }
}

impl Iterator for Ticker {
    type Item = Time;

    fn next(&mut self) -> Option<Self::Item> {
        match self.recv.recv() {
            Ok(v) => Some(v),
            Err(_) => None,
        }
    }
}

impl Iterator for &Ticker {
    type Item = Time;

    fn next(&mut self) -> Option<Self::Item> {
        match self.recv.recv() {
            Ok(v) => Some(v),
            Err(_) => None,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::sleep::sleep;
    use crate::std::time::tick::Ticker;
    use std::sync::Arc;
    use std::time::Duration;

    //test --package mco --lib std::time::tick::test::test_tick -- --exact --nocapture
    #[test]
    fn test_tick() {
        let mut t = Arc::new(Ticker::new(Duration::from_secs(1)));
        let tclone = t.clone();
        co!(move || {
            for x in tclone.as_ref() {
                println!("tick {}", x);
            }
        });
        sleep(Duration::from_secs(3));
        t.stop();
    }
}
