use std::sync::Arc;
use std::time::Duration;
use mco::co;
use mco::coroutine::sleep;
use mco::std::errors::Error;
use mco::std::pool::{Pool, Task};

fn main() {
    let pool = Arc::new(Pool::new(10));
    let copy = pool.clone();
    co!(move ||{
         copy.run();
    });
    for n in 0..100 {
        pool.put(Task::new(move || -> Result<(), Error> {
            sleep(Duration::from_secs(1));
            println!("num:{}", n);
            Ok(())
        }));
    }
    sleep(Duration::from_secs(20));
    //pool.close(); //control pool close
    sleep(Duration::from_secs(5));
    println!("done");
}