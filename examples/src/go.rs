use cogo::coroutine::{Builder, spawn, Spawn, yield_now};
use cogo::go;

fn main() {
    go!(||{
       println!("go");
    });
    go!(2*4096,||{
       println!("go with stack size: {}",cogo::coroutine::current().stack_size());
    });
    (2 * 4096).spawn(|| {
        println!("go with stack size: {}", cogo::coroutine::current().stack_size());
    });
    go!("go",||{
       println!("go with name: {}",cogo::coroutine::current().name().unwrap());
    });
    "go".spawn(|| {
        println!("go with name: {}", cogo::coroutine::current().name().unwrap());
    });
    go!(Builder::new(),||{
       println!("go with Builder");
    });
    Builder::new().spawn(|| {
        println!("go with Builder::spawn");
    });
    spawn(|| {
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
}