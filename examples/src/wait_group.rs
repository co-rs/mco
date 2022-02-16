use std::time::Duration;
use mco::std::sync::WaitGroup;

fn main() {
    let wg = WaitGroup::new();

    //wait thread
    for _ in 0..4 {
        // Create another reference to the wait group.
        let wg = wg.clone();
        std::thread::spawn(move || {
            // Do some work
            println!("sleep 1s");
            mco::coroutine::sleep(Duration::from_secs(1));
            // Drop the reference to the wait group.
            drop(wg);
        });
    }
    //wait coroutines
    for _ in 0..4 {
        // Create another reference to the wait group.
        let wg = wg.clone();

        mco::co!(move || {
        // Do some work.
            println!("sleep 1s");
            mco::coroutine::sleep(Duration::from_secs(1));

        // Drop the reference to the wait group.
        drop(wg);
    });
    }

    // Block until all threads have finished their work.
    wg.wait();
    println!("all done!");
}