use std::time::Duration;


use cogo::coroutine::sleep;
use cogo::{chan, go};
use cogo::std::sync::mpsc;
use cogo::std::sync::mpsc::{bounded, channel, channel_buf, unbounded};


fn main() {
    let (s, r) = chan!();//unbounded
    for i in 0..2 {
        let s_clone = s.clone();
        go!(move ||{
         let t=std::time::Instant::now();
         println!("send{}",i);
         s_clone.send(1);
         println!("send{} done:{:?}",i,t.elapsed());
      });
    }
    println!("remain msg:{}", r.remain());
    println!("sender num:{}", r.sender_num());
    println!("receiver num:{}", r.receiver_num());

    sleep(Duration::from_secs(2));

    for _ in 0..1 {
        let rv = r.recv().unwrap();
        println!("recv = {}", rv);
    }
    println!("chan buffer remain num: {}", r.remain());
    sleep(Duration::from_secs(2));
}