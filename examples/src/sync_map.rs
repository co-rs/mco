use std::ops::Deref;
use std::sync::Arc;
use mco::{co};
use mco::std::sync::{SyncHashMap, WaitGroup};

pub fn main() {
    //or SyncBtreeMap::new_arc()

    let map = SyncHashMap::new_arc();
    let wg = WaitGroup::new();

    for i in 0..100 {
        let m = map.clone();
        let wg = wg.clone();

        co!(move || {
            m.insert(i, i);
            if i == 100 {
                drop(wg);
            }
        });
    }

    wg.wait();
    for (k, v) in map.deref() {
        println!("{},{}", k, v);
    }
}