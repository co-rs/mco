
use cogo::coroutine::sleep;
use cogo::{defer, go};

fn main(){
    defer!(||{
        println!("guard: 1");
    });
    defer!(||{
        println!("guard: 2");
    });
    defer!(||{
        go!(||{
            defer!(||{
                println!("---------------defer in go!-------------");
            });
            println!("go spawn!");
        });
    });
    panic!("None Exception!");
}