#[macro_use]
extern crate cogo;

use std::time::Duration;

use cogo::coroutine;
use cogo::coroutine::sleep;
use cogo::net::TcpListener;

// general select example that use cqueue
fn main() {
    let (tx1, rx1) = chan!();
    let (tx2, rx2) = chan!();
    let listener = TcpListener::bind(("0.0.0.0", 1234)).unwrap();

    go!(move || {
        tx2.send("hello").unwrap();
        sleep(Duration::from_millis(100));
        tx1.send(42).unwrap();
    });

    select! {
        _ = listener.accept() => {
            println!("got connected")
        },
        _ = sleep(Duration::from_millis(1000)) => {

        },
        v = rx1.recv() => {
            println!("rx1 received {:?}",v)
        },
        Ok(v) = rx1.try_recv() => {
            println!("rx1 received {:?}",v)
        },
        a = rx2.recv() => {
            println!("rx2 received, a={:?}", a)
        }
    }

    assert_eq!(rx1.recv(), Ok(42));
}
