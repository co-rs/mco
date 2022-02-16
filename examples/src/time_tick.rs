use mco::coroutine::sleep;
use mco::std::time::tick::Ticker;
use std::sync::Arc;
use std::time::Duration;
use mco::co;

fn main() {
    let mut t = Ticker::new_arc(Duration::from_secs(1));
    let tclone = t.clone();
    co!(move ||{
                for x in tclone.as_ref() {
                   println!("tick {}", x);
                }
    });
    sleep(Duration::from_secs(3));
    t.stop();
}