#![feature(test)]
extern crate test;

use mco::std::sync::WaitGroup;
use mco::std::time::time::Time;
use test::Bencher;

//test bench::single_thread_test ... bench:          44 ns/iter (+/- 1)
#[bench]
fn single_thread_test(b: &mut Bencher) {
    b.iter(|| {
        let now = Time::now();
    });
}

#[bench]
fn mult_thread_test(b: &mut Bencher) {
    std::thread::spawn(move || {
        for _ in 0..100000 {
            let now = Time::now();
        }
    });
    b.iter(|| {
        let now = Time::now();
    });
}
