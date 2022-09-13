use crate::coroutine::spawn;
use crate::std::errors::Error;
use crate::std::sync::{Receiver, Sender};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

pub struct Task {
    pub f: Box<dyn Fn() -> Result<(), Error>>,
}

unsafe impl Send for Task {}

unsafe impl Sync for Task {}

impl Task {
    pub fn new<F>(f: F) -> Task
    where
        F: Fn() -> Result<(), Error> + Send + 'static,
    {
        return Task { f: Box::new(f) };
    }
    pub fn execute(&self) -> Result<(), Error> {
        (self.f)()
    }
}

/// an coroutines pool
pub struct Pool {
    pub worker_num: i32,
    pub idle: (Sender<Option<Task>>, Receiver<Option<Task>>),
    closed: AtomicBool,
}

impl Pool {
    pub fn new(worker_num: i32) -> Self {
        Self {
            worker_num: worker_num,
            idle: chan!(),
            closed: AtomicBool::new(false),
        }
    }

    pub fn new_bounded(worker_num: i32, waiter_num: i32) -> Self {
        Self {
            worker_num: worker_num,
            idle: chan!(waiter_num as usize),
            closed: AtomicBool::new(false),
        }
    }

    pub fn put(&self, task: Task) {
        let _ = self.idle.0.send(Some(task));
    }

    /// close just now
    pub fn close(&self) {
        while self.idle.1.remain() > 0 {
            let _ = self.idle.1.try_recv();
        }
        let _ = self.idle.0.send(None);
    }

    /// close when all task finish
    pub fn close_finish(&self) {
        let _ = self.idle.0.send(None);
    }

    pub fn is_close(&self) -> bool {
        self.closed.load(Ordering::SeqCst)
    }

    pub fn run(&self) {
        let current = Arc::new(chan!(self.worker_num as usize));
        loop {
            match self.idle.1.recv() {
                Ok(task) => match task {
                    None => {
                        log::info!("pool exited");
                        break;
                    }
                    Some(task) => {
                        if let Ok(_) = current.0.send(()) {
                            let rv = current.1.clone();
                            spawn(move || {
                                defer!(move || {
                                    let _ = rv.try_recv();
                                });
                                let r = task.execute();
                                if r.is_err() {
                                    log::error!("task run fail:{}", r.err().unwrap());
                                }
                            });
                        }
                    }
                },
                Err(_) => {
                    log::info!("pool exited");
                    break;
                }
            }
        }
        self.closed.store(true, Ordering::SeqCst);
    }
}

impl Drop for Pool {
    fn drop(&mut self) {
        self.close();
    }
}
