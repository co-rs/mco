use mco::co;
use std::time::Duration;

fn main() {
    println!("thread {:?}", std::thread::current().id());
    co!(|| {
        println!("coroutine run on thread {:?}", std::thread::current().id());
    });
    std::thread::sleep(Duration::from_secs(2));
    // use mco::coroutine::{ spawn, yield_now, Builder, Spawn};
    // co!(2 * 4096, || {
    //     println!(
    //         "coroutine with stack size: {}",
    //         mco::coroutine::current().stack_size()
    //     );
    // });
    // (2 * 4096).spawn(|| {
    //     println!(
    //         "coroutine with stack size: {}",
    //         mco::coroutine::current().stack_size()
    //     );
    // });
    // co!("coroutine", || {
    //     println!(
    //         "coroutine with name: {}",
    //         mco::coroutine::current().name().unwrap_or_default()
    //     );
    // });
    // "coroutine".spawn(|| {
    //     println!(
    //         "coroutine with name: {}",
    //         mco::coroutine::current().name().unwrap_or_default()
    //     );
    // });
    // co!(Builder::new(), || {
    //     println!("coroutine with Builder");
    // });
    // Builder::new().spawn(|| {
    //     println!("coroutine with Builder::spawn");
    // });
    // spawn(|| {
    //     println!("coroutine with method spawn");
    // });
    // //yield example
    // co!(move || {
    //     println!("hi, I'm parent");
    //     let v = (0..100)
    //         .map(|i| {
    //             co!(move || {
    //                 println!("hi, I'm child{:?}", i);
    //                 yield_now();
    //                 println!("bye from child{:?}", i);
    //             })
    //         })
    //         .collect::<Vec<_>>();
    //     yield_now();
    //     // wait child finish
    //     for i in v {
    //         i.join().unwrap();
    //     }
    //     println!("bye from parent");
    // })
    // .join()
    // .unwrap();
    //
    // mco::coroutine::sleep(use std::time::Duration::from_secs(1));
    // //cancel example
    // let g = co!(|| {
    //     mco::defer!(|| { println!("cancel done!") });
    //     for idx in 0..1000 {
    //         mco::coroutine::sleep(use std::time::Duration::from_secs(1));
    //         println!("{}", idx);
    //     }
    // });
    //std::thread::sleep(use std::time::Duration::from_secs(2));
    // g.coroutine().cancel();
}
