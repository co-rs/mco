#[macro_use]
extern crate mco;

use std::time::Duration;

use mco::coroutine;
use mco::mco_gen::Gn;

fn main() {
    coroutine::scope(|scope| {
        co!(scope, || {
            let g = Gn::<()>::new_scoped(|mut scope| {
                let (mut a, mut b) = (0, 1);
                while b < 200 {
                    std::mem::swap(&mut a, &mut b);
                    // sleep in the coroutine context
                    // simulate the timeout event as event iterator
                    coroutine::sleep(Duration::from_millis(100));
                    b = a + b;
                    scope.yield_(b);
                }
                a + b
            });
            g.fold((), |_, i| {
                println!("got {:?}", i);
                // yield_now();
            });
        });
    });
}
