#[macro_use]
extern crate cogo;

use crate::coroutine::yield_now;
use cogo::cogo_gen::Gn;
use cogo::coroutine;

fn main() {
    coroutine::scope(|scope| {
        go!(scope, || {
            let g = cogo::cogo_gen::Gn::<()>::new_scoped(|mut scope| {
                let (mut a, mut b) = (0, 1);
                while b < 200 {
                    std::mem::swap(&mut a, &mut b);
                    // this is yield from the generator context!
                    yield_now();
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
