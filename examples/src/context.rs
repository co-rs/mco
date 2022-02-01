use std::sync::mpsc::RecvError;
use std::time::Duration;
use cogo::{err, go, select};
use cogo::coroutine::sleep;
use cogo::std::context::{CancelCtx, Canceler};
use cogo::std::errors::Error;

//TODO Context is not stable yet
fn main() {
    let mut ctx = CancelCtx::new_arc(None);
    ctx.cancel(Some(err!("EOF")));
    loop {
        let mut break_self = false;
        select! {
                Ok(v) = ctx.done().try_recv() =>{
                    println!("done");
                    break_self = true;
                }
        }
        if break_self {
            break;
        } else {
            println!("sleep 1s");
            sleep(Duration::from_secs(1));
        }
    }
}