use cogo::coroutine::Builder;
use cogo::go;

fn main() {
    go!(||{
       println!("go");
    });
    go!(2*4096,||{
       println!("go with stack size: {}",cogo::coroutine::current().stack_size());
    });
    go!("go",||{
       println!("go with name: {}",cogo::coroutine::current().name().unwrap());
    });
    go!(Builder::new(),||{
       println!("go with Builder");
    });
}