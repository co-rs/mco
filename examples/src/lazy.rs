use std::ops::Deref;
use cogo::go;
use cogo::std::lazy::sync::{Lazy, OnceCell};
use cogo::std::sync::WaitGroup;

static GLOBAL: Lazy<i32> = Lazy::new(|| {
    println!("Init GLOBAL");
    12345678
});

static CELL: OnceCell<i32> = OnceCell::new();

fn main() {
    // // just print init once
    // println!("{}", GLOBAL.deref());
    // println!("{}", GLOBAL.deref());
    // println!("{}", GLOBAL.deref());
    let wg = WaitGroup::new();
    for i in 0..1000 {
        let wgc = wg.clone();
        go!(move ||{
            println!("{} {}",i, GLOBAL.deref());
            drop(wgc);
        });
    }
    wg.wait();//wait done
    CELL.get_or_init(|| {
        println!("Init CELL");
        1
    });
    CELL.get_or_init(|| {
        println!("Init CELL");
        1
    });
}