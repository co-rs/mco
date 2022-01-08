#![cfg(nightly)]
#![feature(test)]
#[macro_use]
extern crate cogo;
extern crate test;

use std::collections::HashMap;
use test::Bencher;
use cogo::std::sync::{Mutex, SyncHashMap};

#[bench]
fn bench_sync_hash_map_write(b: &mut Bencher) {
    let m = SyncHashMap::new();
    let mut i = 0;
    b.iter(|| {
        i += 1;
        m.insert(i, i);
    });
}

#[bench]
fn bench_mutex_hash_map_write(b: &mut Bencher) {
    let m = Mutex::new(HashMap::new());
    let mut i = 0;
    b.iter(|| {
        i += 1;
        m.lock().unwrap().insert(i, i);
    });
}

#[bench]
fn bench_sync_hash_map_read(b: &mut Bencher) {
    let m = SyncHashMap::new();
    for i in 0..1000000{
        m.insert(i, i);
    }
    b.iter(|| {
        m.get(&0);
    });
}

#[bench]
fn bench_mutex_hash_map_read(b: &mut Bencher) {
    let m = Mutex::new(HashMap::new());
    for i in 0..1000000{
        m.lock().unwrap().insert(i, i);
    }
    b.iter(|| {
        m.lock().unwrap().get(&0);
    });
}