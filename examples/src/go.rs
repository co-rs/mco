use std::time::Duration;
use cogo::coroutine::{Builder, sleep, Spawn, yield_now};
use cogo::{defer, go};

fn main() {
    go!(||{
       println!("go");
    });
    go!(2*4096,||{
       println!("go with stack size: {}",cogo::coroutine::current().stack_size());
    });
    (2 * 4096).go(|| {
        println!("go with stack size: {}", cogo::coroutine::current().stack_size());
    });
    go!("go",||{
       println!("go with name: {}",cogo::coroutine::current().name().unwrap_or_default());
    });
    "go".go(|| {
        println!("go with name: {}", cogo::coroutine::current().name().unwrap_or_default());
    });
    go!(Builder::new(),||{
       println!("go with Builder");
    });
    Builder::new().go(|| {
        println!("go with Builder::spawn");
    });
    go(|| {
        println!("go with method spawn");
    });
    go!(move || {
        println!("hi, I'm parent");
        let v = (0..100)
            .map(|i| {
                go!(move || {
                    println!("hi, I'm child{:?}", i);
                    yield_now();
                    println!("bye from child{:?}", i);
                })
            })
            .collect::<Vec<_>>();
        yield_now();
        // wait child finish
        for i in v {
            i.join().unwrap();
        }
        println!("bye from parent");
    }).join().unwrap();

    sleep(Duration::from_secs(1));
    let g = go!(||{
        defer!(||{ println!("cancel done!")});
        for idx in 0..1000{
            sleep(Duration::from_secs(1));
            println!("{}",idx);
        }
    });
    sleep(Duration::from_secs(2));
    g.coroutine().cancel();
}