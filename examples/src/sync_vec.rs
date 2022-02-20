use std::ops::Deref;
use std::sync::Arc;
use mco::{co};
use mco::std::sync::{SyncVec, WaitGroup};

pub fn main() {
    let vec = Arc::new(SyncVec::new());
    let wg = WaitGroup::new();

    for i in 0..100 {
        let m = vec.clone();
        let wg = wg.clone();

        co!(move || {
            m.push(i);
            if i == 100 {
                drop(wg);
            }
        });
    }

    wg.wait();
    for v in vec.deref() {
        println!("{}", v);
    }
}