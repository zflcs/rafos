use core::{sync::atomic::{AtomicU32, Ordering}, future::Future};

use alloc::boxed::Box;
use super::{queue::*, Task, TaskRef, PRIO_LEVEL, TaskType, TaskState};

/// 
#[repr(u32)]
pub enum ExecutorState {
    /// If the `Executor` is `Ready`, it means that there is no `Task` in `Executor`.
    /// So we need to spawn a default `Task`.
    Ready = 1 << 0,
    /// 
    Running = 1 << 1,
}

/// The `Executor` of `async` runtime.
#[repr(C)]
pub struct Executor {
    /// 
    pub state: AtomicU32,
    /// The priority will be updated in these situations:
    /// - spawn_task: fetch_min.
    /// - fetch: it will be set as the priority of task which is fetched now.
    /// - wake: fetch_min.
    priority: AtomicU32,
    /// these queues store tasks according to their priority.
    run_queue: [Queue; PRIO_LEVEL],
    /// thread ids
    stack_poll: [usize; 10],
}

impl Executor {
    ///
    pub const fn new() -> Self {
        Self {
            state: AtomicU32::new(ExecutorState::Ready as _),
            run_queue: [Queue::EMPTY; 8],
            // currents: array_init::array_init(|_| None),
            stack_poll: [usize::MAX; 10],
            priority: AtomicU32::new(u32::MAX),
        }
    }

    /// This will not change the priority immediately
    pub fn set_priority(&self, task_ref: TaskRef, priority: u32) {
        let task = unsafe { &*task_ref.as_ptr() };
        task.update_priority(priority);
    }

    /// spawn a new task in `Executor`
    pub fn spawn(&'static self, fut: Box<dyn Future<Output = i32> + 'static + Send + Sync>, priority: u32, task_type: TaskType) -> TaskRef {
        let task_ref = Task::new(&self, fut, priority, task_type);
        self.run_queue[priority as usize].enqueue(task_ref);
        self.priority.fetch_min(priority, Ordering::Relaxed);
        task_ref
    }

    /// fetch task which has the highest priority
    #[inline(always)]
    pub fn fetch(&mut self) -> Option<TaskRef> {
        for q in &self.run_queue {
            if let Some(task_ref) = q.dequeue() {
                let task = unsafe { &*task_ref.as_ptr() };
                let priority = task.priority.load(Ordering::Relaxed);
                self.priority.store(priority, Ordering::Relaxed);
                return Some(task_ref);
            }
        }
        None
    }


    // ///
    // pub fn wake(&self, task: Arc<Task>) {
    //     let priority = task.priority.load(Ordering::Relaxed);
    //     self.run_queue[priority as usize].enqueue(task);
    //     self.priority.fetch_min(priority, Ordering::Relaxed);
    // }

    /// wake a task according to it's pointer
    #[inline(always)]
    pub fn wake_task_from_ref(&self, task_ref: TaskRef) {
        let task = unsafe { &*task_ref.as_ptr() };
        task.state.store(TaskState::Ready as _, Ordering::Relaxed);
        let priority = task.priority.load(Ordering::Relaxed);
        self.priority.fetch_min(priority, Ordering::Relaxed);
        self.run_queue[priority as usize].enqueue(task_ref);
    }


}
