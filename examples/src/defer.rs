
use cogo::coroutine::sleep;
use cogo::defer;

fn main(){
    defer! {
        println!("guard: 1");
    }
    defer! {
        println!("guard: 2");
    }
    panic!("None Exception!");
}