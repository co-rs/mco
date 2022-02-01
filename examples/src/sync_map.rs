use std::ops::Deref;
use std::sync::Arc;
use cogo::{go};
use cogo::std::sync::{SyncHashMap, WaitGroup};

pub fn main() {
    let m = SyncHashMap::new_arc();//or SyncBtreeMap
    let wg = WaitGroup::new();

    for i in 0..100 {
        let m = m.clone();
        let wg = wg.clone();

        go!(move || {
            m.insert(i, i);
            if i == 100 {
                drop(wg);
            }
        });
    }

    wg.wait();
    for (k, v) in m.deref() {
        println!("{},{}", k, v);
    }
}