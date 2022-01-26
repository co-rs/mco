#![cfg(nightly)]
#![feature(test)]

#[macro_use]
extern crate cogo;
extern crate test;

use test::Bencher;
use cogo::std::sync::channel::channel;


// improve performance  from 39,294 ns/iter to 12,207 ns/iter (my computer)
//test bench_channel  ... bench:      12,207 ns/iter (+/- 118)
#[bench]
fn bench_channel(b: &mut Bencher) {
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
