#![feature(test)]

#[cfg(all(nightly, test))]
mod bench {
    #![feature(test)]
    extern crate test;

    use test::Bencher;
    use cogo::std::sync::WaitGroup;
    use cogo::std::time::time::Time;

    //test bench::single_thread_test ... bench:          44 ns/iter (+/- 1)
    #[bench]
    fn single_thread_test(b: &mut Bencher) {
        b.iter(|| {
            let now = Time::now();
        });
    }

    #[bench]
    fn mult_thread_test(b: &mut Bencher) {
        std::thread::spawn(move ||{
            for _ in 0..100000{
                let now = Time::now();
            }
        });
        b.iter(|| {
            let now = Time::now();
        });
    }
}