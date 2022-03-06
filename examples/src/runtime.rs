use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::thread::sleep;
use std::time::Duration;
use mco::{co};


fn main() {
    pub struct A {
        f: fn(),
    }

    impl Future for A {
        type Output = ();

        fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
            //yield
            let mut f = Box::pin(tokio::task::yield_now());
            loop {
                match Pin::new(&mut f).poll(cx) {
                    Poll::Ready(v) => {
                        break;
                    }
                    Poll::Pending => {}
                }
            }
            return Poll::Ready((self.f)());
        }
    }
    mco::get_runtime().spawn(A {
        f: || {
            println!("hay1");
        }
    });
}