use cogo::coroutine::sleep;
use cogo::std::time::tick::Ticker;
use std::sync::Arc;
use std::time::Duration;
use cogo::go;

fn main() {
    let mut t = Arc::new(Ticker::new(Duration::from_secs(1)));
    let tclone = t.clone();
    go!(move ||{
                for x in tclone.as_ref() {
                   println!("tick {}", x);
                }
       });
    sleep(Duration::from_secs(3));
    t.stop();
}