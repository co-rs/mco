#![feature(test)]

#[cfg(all(nightly, test))]
mod bench {
    #![cfg(nightly)]
    extern crate test;

    use std::sync::Arc;
    use std::sync::mpsc::channel;
    use self::test::Bencher;

    use std::thread;
    use cogo::chan;
    use cogo::std::queue::mpmc_bounded::Queue;

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

    #[cfg(test)]
    mod test_queue {
        pub trait ScBlockPop<T> {
            fn block_pop(&self) -> T;
        }

        macro_rules! block_pop_sc_impl {
        // `()` indicates that the macro takes no argument.
        ($queue:path) => {
            impl<T> ScBlockPop<T> for cogo::std::queue::spsc::Queue<T> {
                fn block_pop(&self) -> T {
                    let mut i = 0;
                    loop {
                        if let Some(v) = self.pop() {
                            return v;
                        }

                        if i > 10 {
                            i = 0;
                            ::std::thread::yield_now();
                        }
                        i += 1;
                    }
                }
            }
        };
     }

        block_pop_sc_impl!(cogo::std::queue::spsc);
    }

    use test_queue::ScBlockPop;


    #[test]
    fn spsc_peek() {
        let q = Queue::new();
        assert_eq!(unsafe { q.peek() }, None);
        q.push(1);
        assert_eq!(unsafe { q.peek() }, Some(&1));
        let v = q.pop();
        assert_eq!(v, Some(1));
        assert_eq!(unsafe { q.peek() }, None);
    }

    //test bench::multi_1p1c_test     ... bench:  12,646,720 ns/iter (+/- 1,005,163)
    #[bench]
    fn bulk_pop_1p1c_bench(b: &mut Bencher) {
        b.iter(|| {
            let q = Arc::new(Queue::new());
            let total_work: usize = 1000_000;
            // create worker threads that generate mono increasing index
            let _q = q.clone();
            // in other thread the value should be still 100
            thread::spawn(move || {
                for i in 0..total_work {
                    _q.push(i);
                }
            });

            let mut size = 0;
            let mut vec = Vec::with_capacity(total_work);
            while size < total_work {
                size += q.bulk_pop(&mut vec);
            }

            for (i, item) in vec.iter().enumerate() {
                assert_eq!(i, *item);
            }
        });
    }

    #[bench]
    fn single_thread_test(b: &mut Bencher) {
        let q = Queue::new();
        let mut i = 0;
        b.iter(|| {
            q.push(i);
            assert_eq!(q.pop(), Some(i));
            i += 1;
        });
    }

    #[bench]
    fn multi_1p1c_test(b: &mut Bencher) {
        b.iter(|| {
            let q = Arc::new(Queue::new());
            let total_work: usize = 1000_000;
            // create worker threads that generate mono increasing index
            let _q = q.clone();
            // in other thread the value should be still 100
            thread::spawn(move || {
                for i in 0..total_work {
                    _q.push(i);
                }
            });

            for i in 0..total_work {
                let v = q.block_pop();
                assert_eq!(i, v);
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