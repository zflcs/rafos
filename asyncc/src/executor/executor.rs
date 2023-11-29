use core::{sync::atomic::{AtomicU32, Ordering}, future::Future};

use super::{queue::*, Task, TaskRef, PRIO_LEVEL, TaskType, TaskState};

/// The `Executor` of `async` runtime.
#[repr(C)]
pub struct Executor {
    /// this queue uses `FIFO` scheduling mechanism no matter what priority the inner task is.
    /// Once there are tasks in this queue, all the tasks in `RunQueue` should be executed later.
    wake_queue: Queue,
    /// The priority will be updated in these situations:
    /// - spawn_task: fetch_min.
    /// - fetch: it will be set as the priority of task which is fetched now.
    /// - wake: fetch_min.
    priority: AtomicU32,
    /// these queues store tasks according to their priority.
    run_queue: [Queue; PRIO_LEVEL],
    /// current task
    currents: [Option<TaskRef>; 10],
    /// thread ids
    threads: [usize; 10],
}

impl Executor {
    ///
    pub const fn new() -> Self {
        Self {
            wake_queue: Queue::EMPTY,
            run_queue: [Queue::EMPTY; 8],
            currents: [None; 10],
            threads: [usize::MAX; 10],
            priority: AtomicU32::new(u32::MAX),
        }
    }

    /// This will not change the priority immediately
    pub fn set_priority(&self, task_ref: TaskRef, priority: u32) {
        let task = unsafe { &*task_ref.as_ptr() };
        task.update_priority(priority);
    }

    /// spawn a new task in `Executor`
    pub fn spawn(&'static self, fut: impl Future<Output = i32> + 'static + Send + Sync, priority: u32, task_type: TaskType) -> TaskRef {
        let task_ref = Task::new(&self, fut, priority, task_type);
        self.run_queue[priority as usize].enqueue(task_ref);
        self.priority.fetch_min(priority, Ordering::Relaxed);
        task_ref
    }

    /// fetch task which has the highest priority
    pub fn fetch(&mut self, tid: usize) -> Option<TaskRef> {
        assert!(tid < 10);
        if let Some(task_ref) = self.wake_queue.dequeue() {
            let task = unsafe { &*task_ref.as_ptr() };
            let priority = task.priority.load(Ordering::Relaxed);
            self.priority.store(priority, Ordering::Relaxed);
            self.currents[tid] = Some(task_ref);
            return Some(task_ref);
        }
        for q in &self.run_queue {
            if let Some(task_ref) = q.dequeue() {
                let task = unsafe { &*task_ref.as_ptr() };
                let priority = task.priority.load(Ordering::Relaxed);
                self.priority.store(priority, Ordering::Relaxed);
                self.currents[tid] = Some(task_ref);
                return Some(task_ref);
            }
        }
        None
    }

    ///
    pub fn add_wait_tid(&mut self, tid: usize) {
        for i in self.threads {
            if self.threads[i] != usize::MAX {
                self.threads[i] = tid;
            }
        }
    }

    /// wake a task according to it's pointer
    pub fn wake_task_from_ref(&self, task_ref: TaskRef) {
        let task = unsafe { &*task_ref.as_ptr() };
        task.state.store(TaskState::Ready as _, Ordering::Relaxed);
        let priority = task.priority.load(Ordering::Relaxed);
        self.priority.fetch_min(priority, Ordering::Relaxed);
        self.wake_queue.enqueue(task_ref);
    }

    /// get the current task on the thread
    pub fn current_task(&self, tid: usize) -> TaskRef {
        assert!(tid < 10);
        self.currents[tid].unwrap()
    }
}
