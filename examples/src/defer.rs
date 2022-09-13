use mco::coroutine::sleep;
use mco::{co, defer};
use std::cell::RefCell;

fn main() {
    defer! {
        println!("guard: 1");
    }
    defer!(|| {
        println!("guard: 2");
    });
    defer! {
        println!("guard: 3");
    }
    let mut complete = false;
    defer!(move || {
        if complete.eq(&false) {
            complete = true;
            println!("tx complete");
        }
    });
    //if panic,defer will make sure complete
    if true {
        panic!("None Exception!");
    }
    complete = true;
}
