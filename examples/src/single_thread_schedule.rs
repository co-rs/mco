#[macro_use]
extern crate mco;

use std::io::{Read, Write};

use mco::coroutine;
use mco::net::{TcpListener, TcpStream};

fn main() {
    // below config would schedule all the coroutines
    // on the single worker thread
    mco::config().set_workers(1);

    // start the server
    let _server = co!(|| {
        let listener = TcpListener::bind("0.0.0.0:8000").unwrap();
        while let Ok((mut stream, _)) = listener.accept() {
            co!(move || {
                let mut buf = vec![0; 1024 * 8]; // alloc in heap!
                while let Ok(n) = stream.read(&mut buf) {
                    if n == 0 {
                        break;
                    }
                    stream.write_all(&buf[0..n]).unwrap();
                }
            });
        }
    });

    // run some client until all finish
    coroutine::scope(|s| {
        for i in 0..100 {
            co!(s, move || {
                let mut buf = [i; 100];
                let mut conn = TcpStream::connect("0.0.0.0:8000").unwrap();
                conn.write_all(&buf).unwrap();
                conn.read_exact(&mut buf).unwrap();
                for v in buf.iter() {
                    assert_eq!(*v, i);
                }
            });
        }
    });
}
