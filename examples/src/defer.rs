use cogo::coroutine::sleep;
use cogo::{defer, go};

fn main() {
    defer!({
        println!("guard: 1");
    });
    defer!(||{
        println!("guard: 2");
    });
    defer! {
        println!("guard: 3");
    }
    defer! {
        go!(||{
            println!("go spawn!");
        });
    }
    ;
    panic!("None Exception!");
}