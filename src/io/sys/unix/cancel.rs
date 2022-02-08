use std::sync::atomic::Ordering;
use std::sync::Arc;

use super::EventData;
use crate::cancel::CancelIo;
use crate::scheduler::get_scheduler;
use crate::std::sync::AtomicOption;

pub struct CancelIoImpl(AtomicOption<Arc<EventData>>);

impl CancelIo for CancelIoImpl {
    type Data = Arc<EventData>;

    fn new() -> Self {
        CancelIoImpl(AtomicOption::none())
    }

    fn set(&self, data: Arc<EventData>) {
        self.0.swap(data);
    }

    fn clear(&self) {
        self.0.take();
    }

    fn cancel(&self) -> Result<(),std::io::Error> {
        if let Some(e) = self.0.take() {
            if let Some(co) = e.co.take() {
                get_scheduler().schedule(co);
            }
        }
        Ok(())
    }
}
