use std::time::Duration;
use cogo::coroutine::sleep;
use cogo::go;
use cogo::std::sync::mpsc::{channel, channel_buf};


fn main() {
    let (s, r) = channel_buf(3);
    let s1=s.clone();
    go!(move ||{
         let t=std::time::Instant::now();
         println!("send1");
         s1.send(1);
         println!("send1 done:{:?}",t.elapsed());
    });
    let s2=s.clone();
    go!(move ||{
         let t=std::time::Instant::now();
         println!("send2");
         s2.send(1);
         println!("send2 done:{:?}",t.elapsed());
    });
    let s3=s.clone();
    go!(move ||{
         let t=std::time::Instant::now();
         println!("send3");
         s3.send(1);
         println!("send3 done:{:?}",t.elapsed());
    });
    sleep(Duration::from_secs(2));
    let rv = r.recv().unwrap();
    println!("recv = {}", rv);
    let rv = r.recv().unwrap();
    println!("recv = {}", rv);
    sleep(Duration::from_secs(2));
}