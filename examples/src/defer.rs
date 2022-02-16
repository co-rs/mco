use mco::coroutine::sleep;
use mco::{defer, co};

fn main() {
    defer!{
        println!("guard: 1");
    };
    defer!(||{
        println!("guard: 2");
    });
    defer! {
        println!("guard: 3");
    }
    defer! {
        co!(||{
            println!("-----------------go spawn success-------------------");
        });
    }
    panic!("None Exception!");
}