use std::ops::Deref;
use std::sync::Arc;
use mco::{co, sync_btree_map, sync_hash_map};
use mco::std::sync::{SyncHashMap, WaitGroup};
pub fn main() {
    //or SyncBtreeMap
    let btree = sync_btree_map!{1:1};
    let map = Arc::new(sync_hash_map!{});
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