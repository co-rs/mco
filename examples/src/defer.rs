use std::cell::RefCell;
use mco::coroutine::sleep;
use mco::{defer, co};

fn main() {
    defer! {
        println!("guard: 1");
    }
    defer!(||{
        println!("guard: 2");
    });
    defer! {
        println!("guard: 3");
    }
    let complete = RefCell::new(false);
    let c = complete.clone();
    defer!(move|| {
        if c.borrow().eq(&false){
            *c.borrow_mut() = true;
            println!("tx complete");
        }
    });
    //if panic,defer will make sure complete
    if true {
        panic!("None Exception!");
    }
    *complete.borrow_mut() = true;
}