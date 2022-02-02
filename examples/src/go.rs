use cogo::coroutine::{Builder, spawn, Spawn};
use cogo::go;

fn main() {
    go!(||{
       println!("go");
    });
    go!(2*4096,||{
       println!("go with stack size: {}",cogo::coroutine::current().stack_size());
    });
    (2 * 4096).spawn(|| {
        println!("go with stack size: {}", cogo::coroutine::current().stack_size());
    });
    go!("go",||{
       println!("go with name: {}",cogo::coroutine::current().name().unwrap());
    });
    "go".spawn(|| {
        println!("go with name: {}", cogo::coroutine::current().name().unwrap());
    });
    go!(Builder::new(),||{
       println!("go with Builder");
    });
    Builder::new().spawn(|| {
        println!("go with Builder::spawn");
    });
    spawn(|| {
        println!("go with method spawn");
    });
}