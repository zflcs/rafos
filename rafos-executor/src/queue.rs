/// This mod defines some queue in the `Executor`
use crate::TaskRef;

use crossbeam::queue::SegQueue;

/// This queue stores the `Arc<Task>` which is ready to run.
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
    pub fn dequeue(&self) -> Option<TaskRef> {
        self.0.pop()
    }

    ///
    pub fn enqueue(&self, task: TaskRef) {
        self.0.push(task);
    }
}
