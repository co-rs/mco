use std::time::Duration;
use cogo::coroutine::sleep;
use cogo::{chan, go};

fn main() {
    //unbounded
    let (s, r) = chan!();
    s.send(1);
    println!("remain msg:{},sender num:{},receiver num:{}", r.remain(), r.sender_num(), r.receiver_num());
    let rv = r.recv().unwrap();
    println!("recv = {},remain:{}", rv, r.remain());

    sleep(Duration::from_secs(1));

    //bounded length, If the sender sends more messages than the limit, it waits until the message is consumed
    let (s, r) = chan!(1);
    go!(move ||{
       let send_result = s.send(1);
       println!("send 1 is_ok:{}", send_result.is_ok());
       //will blocking until the excess messages are consumed or the channel is closed
       println!("s.send(2) blocking 2s...");
       let send_result = s.send(2);
       println!("send 2 is_ok: {:?}", send_result);
    });

    sleep(Duration::from_secs(2));
    let rv = r.recv().unwrap();
    println!("recv = {},remain:{}", rv, r.remain());
    let rv = r.recv().unwrap();
    println!("recv = {},remain:{}", rv, r.remain());
    sleep(Duration::from_secs(1));
}
