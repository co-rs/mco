use std::sync::mpsc::RecvError;
use cogo::{go, select};
use cogo::std::context::{CancelCtx, Canceler};
use cogo::std::errors::Error;

fn main() {
    let mut c = CancelCtx::new(None);
    // let f=move ||{
    //     loop{
    //         select! {
    //             v = c.done().unwrap()
    //         }
    //     }
    // };
    // go!(f);
    c.cancel(Some(Error::from("EOF")));

}