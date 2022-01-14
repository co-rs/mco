use std::ops::Deref;
use cogo::std::lazy::sync::Lazy;

static GLOBAL: Lazy<i32> = Lazy::new(|| {
    println!("init");
    12345678
});

fn main() {
    // just print init once
    println!("{}", GLOBAL.deref());
    println!("{}", GLOBAL.deref());
    println!("{}", GLOBAL.deref());
}