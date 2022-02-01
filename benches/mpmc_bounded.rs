#![feature(test)]

#[cfg(all(nightly, test))]
mod bench {
    #![cfg(nightly)]
    extern crate test;

    use self::test::Bencher;


    use std::sync::mpsc::channel;
    use std::thread;
    use cogo::std::queue::mpmc_bounded::Queue;

    #[bench]
    fn bounded_mpmc(b: &mut Bencher) {
        b.iter(|| {
            let total_work = 1000_000;
            let nthreads = 1;
            let nmsgs = total_work / nthreads;
            let q = Queue::with_capacity(nthreads * nmsgs);
            assert_eq!(None, q.pop());
            let (tx, rx) = channel();

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
}