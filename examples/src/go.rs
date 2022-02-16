use std::time::Duration;
use mco::coroutine::{Builder, sleep, Spawn, spawn, yield_now};
use mco::{defer, co};

fn main() {
    co!(||{
       println!("go");
    });
    co!(2*4096,||{
       println!("go with stack size: {}",mco::coroutine::current().stack_size());
    });
    (2 * 4096).spawn(|| {
        println!("go with stack size: {}", mco::coroutine::current().stack_size());
    });
    co!("go",||{
       println!("go with name: {}",mco::coroutine::current().name().unwrap_or_default());
    });
    "go".spawn(|| {
        println!("go with name: {}", mco::coroutine::current().name().unwrap_or_default());
    });
    co!(Builder::new(),||{
       println!("go with Builder");
    });
    Builder::new().spawn(|| {
        println!("go with Builder::spawn");
    });
    spawn(|| {
        println!("go with method spawn");
    });
    co!(move || {
        println!("hi, I'm parent");
        let v = (0..100)
            .map(|i| {
                co!(move || {
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
    let g = co!(||{
        defer!(||{ println!("cancel done!")});
        for idx in 0..1000{
            sleep(Duration::from_secs(1));
            println!("{}",idx);
        }
    });
    sleep(Duration::from_secs(2));
    g.coroutine().cancel();
}