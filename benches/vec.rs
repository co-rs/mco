#![cfg(nightly)]
#![feature(test)]
#[macro_use]
extern crate mco;
extern crate test;

use test::Bencher;
use mco::std::sync::{Mutex, SyncVec};

#[bench]
fn bench_sync_vec_push(b: &mut Bencher) {
    let m = SyncVec::new();
    let mut i = 0;
    b.iter(|| {
        i += 1;
        m.push(i);
    });
}

#[bench]
fn bench_mutex_vec_push(b: &mut Bencher) {
    let m = Mutex::new(Vec::new());
    let mut i = 0;
    b.iter(|| {
        i += 1;
        m.lock().unwrap().push(i);
    });
}

#[bench]
fn bench_sync_vec_read(b: &mut Bencher) {
    let m = SyncVec::new();
    for i in 0..1000000 {
        m.push(i);
    }
    b.iter(|| {
        m.get(0);
    });
}

#[bench]
fn bench_mutex_vec_read(b: &mut Bencher) {
    let m = Mutex::new(Vec::new());
    for i in 0..1000000 {
        m.lock().unwrap().push(i);
    }
    b.iter(|| {
        m.lock().unwrap().get(0);
    });
}

#[bench]
fn bench_sync_vec_iter(b: &mut Bencher) {
    let m = SyncVec::new();
    for i in 0..1000000 {
        m.push(i);
    }
    b.iter(|| {
        for x in &m {

        }
    });
}