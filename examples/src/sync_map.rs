use std::ops::Deref;
use std::sync::Arc;
use cogo::{go};
use cogo::std::sync::{SyncHashMap, WaitGroup};

pub fn main() {
    let m = Arc::new(SyncHashMap::new());
    let wg = WaitGroup::new(); //notice message

    for i in 0..100 {
        let m1 = m.clone();
        let wg1 = wg.clone();
        //many coroutine insert the SyncMap
        go!(move ||{
           m1.insert(i,i);
           if i==100{
                drop(wg1);
            }
       });
    }

    wg.wait();
    for (k, v) in m.deref() {
        println!("{},{}", k, v);
    }
}