#![allow(deprecated)]

use mco_gen::*;

fn xrange(start: u32, end: u32) -> u32 {
    for i in start..end {
        yield_with(i);
    }
    done!();
}

fn main() {
    let g1 = Gn::new_opt(4096,|| xrange(0, 10));
    let g2 = Gn::new_opt(4096,|| xrange(10, 20));

    let g = Gn::new_scoped(4096,|mut s| {
        s.yield_from(g1);
        s.yield_from(g2);
        done!();
    });

    g.fold(0, |sum, x| {
        println!("i={}, sum={}", x, sum + x);
        sum + x
    });
}
