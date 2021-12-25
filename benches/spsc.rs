#[cfg(all(nightly, test))]
mod bench {
    #![feature(test)]
    extern crate test;
    use self::test::Bencher;

    use super::*;
    use std::sync::mpsc::channel;
    use std::sync::Arc;
    use std::thread;
    use cogo::std::queue::spsc::Queue;

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
}