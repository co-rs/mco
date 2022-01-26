use std::time::Duration;
use cogo::coroutine::sleep;
use cogo::go;
use cogo::std::sync::channel::channel;
use cogo::std::sync::WaitGroup;

#[test]
fn channel_recv() {
    let wait_group = WaitGroup::new();
    let (s,r)=channel();
    for _ in 0..500{
        let r1=r.clone();
        let s1=s.clone();
        let w = wait_group.clone();
        go!(move ||{
            s1.send(1);
            r1.recv().unwrap();
            drop(w);
        });
    }
    for idx in 0..500{
        let w = wait_group.clone();
        let r1=r.clone();
        let s1=s.clone();
        go!(move ||{
            if idx==499{
                println!("sleep 5s");
                sleep(Duration::from_secs(5));
            }
            s1.send(1);
            r1.recv().unwrap();
            println!("recv");
            drop(w);
        });
    }
    wait_group.wait();
}