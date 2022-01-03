use std::time::Duration;
use cogo::coroutine::sleep;
use cogo::go;
use cogo::std::sync::mpsc::{channel, channel_buf};


fn main() {
    let (s, r) = channel_buf(1);
    go!(move ||{
         let t=std::time::Instant::now();
         println!("send");
         s.send(1);
         println!("send done:{:?}",t.elapsed());
    });
    sleep(Duration::from_secs(5));
    let rv = r.recv().unwrap();
    println!("recv = {}", rv);
    sleep(Duration::from_secs(5));
}