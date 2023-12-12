#![feature(test)]
extern crate test;

use test::Bencher;
use mco_gen::Stack;

//windows 2404 ns/iter (+/- 306)
#[bench]
fn bench_stack_new(b: &mut Bencher) {
    b.iter(|| {
        let s = Stack::new(4096);
    });
}