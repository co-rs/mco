#![cfg(nightly)]
#![feature(test)]

#[macro_use]
extern crate cogo;
extern crate test;

use crate::coroutine::*;
use cogo::{config, coroutine};
use test::Bencher;
use cogo::std::channel::unbounded;
use cogo::std::sync::mpsc::channel;

#[bench]
fn yield_bench(b: &mut Bencher) {
    // don't print any panic info
    // when cancel the generator
    // panic::set_hook(Box::new(|_| {}));
    b.iter(|| {
        let h = go!(|| for _i in 0..10000 {
            yield_now();
        });

        h.join().unwrap();
    });
}

#[bench]
fn spawn_bench(b: &mut Bencher) {
    //test spawn_bench    ... bench:   1,731,980 ns/iter (+/- 203,088)
    config().set_work_steal(true);
    //test spawn_bench    ... bench:     657,233 ns/iter (+/- 38,624)
    config().set_work_steal(false);

    b.iter(|| {
        let total_work = 1000;
        let threads = 2;
        let mut vec = Vec::with_capacity(threads);
        for _t in 0..threads {
            let j = std::thread::spawn(move || {
                scope(|scope| {
                    for _i in 0..total_work / threads {
                        go!(scope, || {
                            // yield_now();
                        });
                    }
                });
            });
            vec.push(j);
        }
        for j in vec {
            j.join().unwrap();
        }
    });
}

#[bench]
fn spawn_bench_1(b: &mut Bencher) {
    cogo::config().set_pool_capacity(10000);
    b.iter(|| {
        let total_work = 1000;
        let threads = 2;
        let mut vec = Vec::with_capacity(threads);
        for _t in 0..threads {
            let work = total_work / threads;
            let j = std::thread::spawn(move || {
                let v = (0..work).map(|_| go!(|| {})).collect::<Vec<_>>();
                for h in v {
                    h.join().unwrap();
                }
            });
            vec.push(j);
        }
        for j in vec {
            j.join().unwrap();
        }
    });
}

#[bench]
fn smoke_bench(b: &mut Bencher) {
    cogo::config().set_pool_capacity(10000);
    b.iter(|| {
        let threads = 5;
        let mut vec = Vec::with_capacity(threads);
        for _t in 0..threads {
            let j = std::thread::spawn(|| {
                scope(|scope| {
                    for _i in 0..200 {
                        go!(scope, || for _j in 0..1000 {
                            yield_now();
                        });
                    }
                });
            });
            vec.push(j);
        }
        for j in vec {
            j.join().unwrap();
        }
    });
}

#[bench]
fn smoke_bench_1(b: &mut Bencher) {
    cogo::config().set_pool_capacity(10000);
    b.iter(|| {
        let threads = 5;
        let mut vec = Vec::with_capacity(threads);
        for _t in 0..threads {
            let j = std::thread::spawn(|| {
                scope(|scope| {
                    for _i in 0..2000 {
                        go!(scope, || for _j in 0..4 {
                            yield_now();
                        });
                    }
                });
            });
            vec.push(j);
        }
        for j in vec {
            j.join().unwrap();
        }
    });
}

#[bench]
fn smoke_bench_2(b: &mut Bencher) {
    cogo::config().set_pool_capacity(10000);
    b.iter(|| {
        scope(|s| {
            // create a main coroutine, let it spawn 10 sub coroutine
            for _ in 0..100 {
                go!(s, || {
                    scope(|ss| {
                        for _ in 0..100 {
                            go!(ss, || {
                                // each task yield 4 times
                                for _ in 0..4 {
                                    yield_now();
                                }
                            });
                        }
                    });
                });
            }
        });
    });
}

#[bench]
fn smoke_bench_3(b: &mut Bencher) {
    b.iter(|| {
        let mut vec = Vec::with_capacity(100);
        // create a main coroutine, let it spawn 10 sub coroutine
        for _ in 0..100 {
            let j = go!(|| {
                let mut _vec = Vec::with_capacity(100);
                for _ in 0..100 {
                    let _j = go!(|| {
                        // each task yield 10 times
                        for _ in 0..4 {
                            yield_now();
                        }
                    });
                    _vec.push(_j);
                }
                for _j in _vec {
                    _j.join().ok();
                }
            });
            vec.push(j);
        }
        for j in vec {
            j.join().ok();
        }
    });
}


// improve performance  from 39,294 ns/iter to 12,207 ns/iter (my computer)
//test bench_channel  ... bench:      12,207 ns/iter (+/- 118)
#[bench]
fn bench_channel(b: &mut Bencher) {
    b.iter(|| {
        let (s,r) = unbounded();
        for _ in 0..1000{
            s.send(1);
        }
        for _ in 0..1000{
            let r=r.recv().unwrap();
        }
    });
}

//test bench_channel2 ... bench:      39,294 ns/iter (+/- 639)
#[bench]
fn bench_channel2(b: &mut Bencher) {
    b.iter(|| {
        let (s,r) = channel();
        for _ in 0..1000{
            s.send(1);
        }
        for _ in 0..1000{
            let r=r.recv().unwrap();
        }
    });
}
