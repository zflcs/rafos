/// This mod defines some queue in the `Executor`
use crate::TaskRef;

use crossbeam::queue::SegQueue;

/// This queue stores the `TaskRef` which is ready to run.
#[repr(transparent)]
pub struct Queue(SegQueue<TaskRef>);

impl Queue {
    pub const EMPTY: Self = Self(SegQueue::new());
    ///
    #[allow(unused)]
    pub const fn new() -> Self {
        Self(SegQueue::new())
    }

    ///
    #[inline(always)]
    pub fn dequeue(&self) -> Option<TaskRef> {
        self.0.pop()
    }

    ///
    #[inline(always)]
    pub fn enqueue(&self, task_ref: TaskRef) {
        self.0.push(task_ref);
    }
}
