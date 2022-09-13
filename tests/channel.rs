use mco::co;
use mco::coroutine::sleep;
use mco::std::sync::channel::channel;
use mco::std::sync::WaitGroup;
use std::time::Duration;

#[test]
fn channel_recv() {
    let wait_group = WaitGroup::new();
    let (s, r) = channel();
    for _ in 0..500 {
        let r1 = r.clone();
        let s1 = s.clone();
        let w = wait_group.clone();
        co!(move || {
            s1.send(1);
            r1.recv().unwrap();
            drop(w);
        });
    }
    for idx in 0..500 {
        let w = wait_group.clone();
        let r1 = r.clone();
        let s1 = s.clone();
        co!(move || {
            if idx == 499 {
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
