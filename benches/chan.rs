#![feature(test)]

#[cfg(all(nightly, test))]
mod bench {
    #![cfg(nightly)]
    extern crate test;

    use std::sync::Arc;
    use std::sync::mpsc::channel;
    use self::test::Bencher;

    use std::thread;
    use mco::chan;
    use mco::std::queue::mpmc_bounded::Queue;

    #[bench]
    fn bounded_mpmc(b: &mut Bencher) {
        b.iter(|| {
            let total_work = 1000_000;
            let nthreads = 1;
            let nmsgs = total_work / nthreads;
            let q = Queue::with_capacity(nthreads * nmsgs);
            assert_eq!(None, q.pop());
            let (tx, rx) = chan!();

            for _ in 0..nthreads {
                let q = q.clone();
                let tx = tx.clone();
                thread::spawn(move || {
                    let q = q;
                    for i in 0..nmsgs {
                        assert!(q.push(i).is_ok());
                    }
                    tx.send(()).unwrap();
                });
            }

            let mut completion_rxs = vec![];
            for _ in 0..nthreads {
                let (tx, rx) = channel();
                completion_rxs.push(rx);
                let q = q.clone();
                thread::spawn(move || {
                    let q = q;
                    let mut i = 0;
                    loop {
                        match q.pop() {
                            None => {}
                            Some(_) => {
                                i += 1;
                                if i == nmsgs {
                                    break;
                                }
                            }
                        }
                    }
                    tx.send(i).unwrap();
                });
            }

            for rx in completion_rxs.iter_mut() {
                assert_eq!(nmsgs, rx.recv().unwrap());
            }
            for _ in 0..nthreads {
                rx.recv().unwrap();
            }
        });
    }


    // #[bench]
    // the channel bench result show that it's 10 fold slow than our queue
    // not to mention the multi core contention
    #[allow(dead_code)]
    fn sys_stream_test(b: &mut Bencher) {
        b.iter(|| {
            let (tx, rx) = channel();
            let total_work: usize = 1000_000;
            // create worker threads that generate mono increasing index
            // in other thread the value should be still 100
            thread::spawn(move || {
                for i in 0..total_work {
                    tx.send(i).unwrap();
                }
            });

            for i in 0..total_work {
                assert_eq!(i, rx.recv().unwrap());
            }
        });
    }


    // improve performance  from 39,294 ns/iter to 12,207 ns/iter (my computer)
    //test bench_channel  ... bench:      12,207 ns/iter (+/- 118)
    #[bench]
    fn bench_channel(b: &mut Bencher) {
        b.iter(|| {
            let (s, r) = chan!();
            for _ in 0..1000 {
                s.send(1);
            }
            for _ in 0..1000 {
                let r = r.recv().unwrap();
            }
        });
    }
}