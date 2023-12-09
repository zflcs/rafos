/// This mod defines some queue in the `Executor`
use crate::TaskRef;

// use crossbeam::queue::SegQueue;
use heapless::mpmc::MpMcQueue;

/// This queue stores the `TaskRef` which is ready to run.
#[repr(transparent)]
pub struct Queue(MpMcQueue<TaskRef, 128>);

impl Queue {
    pub const EMPTY: Self = Self(MpMcQueue::new());
    ///
    #[allow(unused)]
    pub const fn new() -> Self {
        Self(MpMcQueue::new())
    }

    ///
    #[inline(always)]
    pub fn dequeue(&self) -> Option<TaskRef> {
        self.0.dequeue()
    }

    ///
    #[inline(always)]
    pub fn enqueue(&self, task_ref: TaskRef) {
        while self.0.enqueue(task_ref).is_err() {}
    }
}
