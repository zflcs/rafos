use alloc::{
    collections::{vec_deque, VecDeque},
    sync::Arc, vec, string::{ToString, String},
};
use config::{CPU_NUM, ROOT_DIR, INIT_TASK_PATH};
use console::hart_id;
use kernel_sync::{CPUs, SpinLock};
use spin::Lazy;
use crate::{task::*, loader::from_args};


/// Possible interfaces for task schedulers.
pub trait Scheduler {
    /// Add a task to be scheduled sooner or later.
    fn add(&mut self, task: Arc<Task>);

    /// Get a task to run on the target processor.
    fn fetch(&mut self) -> Option<Arc<Task>>;
}

pub struct QueueScheduler {
    queue: VecDeque<Arc<Task>>,
}

impl QueueScheduler {
    pub fn new() -> Self {
        Self {
            queue: VecDeque::new(),
        }
    }

    /// Returns a front-to-back iterator that returns immutable references.
    pub fn iter(&self) -> vec_deque::Iter<Arc<Task>> {
        self.queue.iter()
    }
}

impl Scheduler for QueueScheduler {
    fn add(&mut self, task: Arc<Task>) {
        self.queue.push_back(task);
    }

    fn fetch(&mut self) -> Option<Arc<Task>> {
        if self.queue.is_empty() {
            return None;
        }
        let task = self.queue.pop_front().unwrap();

        // State cannot be set to other states except [`TaskState::Runnable`] by other harts,
        // e.g. this task is waken up by another task that releases the resources.
        if *task.state.lock() != TaskState::RUNNABLE {
            self.queue.push_back(task);
            None
        } else {
            Some(task)
        }
    }
}

/// Reserved for future SMP usage.
pub struct CPUContext {
    /// Current task.
    pub curr: Option<Arc<Task>>,

    /// Idle task context.
    pub idle_ctx: TaskContext,
}

impl CPUContext {
    /// A hart joins to run tasks
    pub const fn new() -> Self {
        Self {
            curr: None,
            idle_ctx: TaskContext::zero(),
        }
    }
}

/// Global task manager shared by CPUs.
pub static TASK_MANAGER: Lazy<SpinLock<QueueScheduler>> =
    Lazy::new(|| {
        let mut scheduler = QueueScheduler::new();
        if let Some(task) = from_args(String::from(ROOT_DIR), vec![INIT_TASK_PATH.to_string()])
            .map_err(|_| log::warn!("INIT TASK NOT FOUND"))
            .ok()
        {
            scheduler.add(task);
        }
        SpinLock::new(scheduler)
    });

const EMPTY_CPU: CPUContext = CPUContext::new();
/// Global cpu local states.
pub static mut CPU_LIST: [CPUContext; CPU_NUM] = [EMPTY_CPU; CPU_NUM];


/// Returns this cpu context.
pub fn cpu() -> &'static mut CPUContext {
    unsafe { &mut CPU_LIST[hart_id()] }
}

/// Gets current task context.
///
/// # Safety
///
/// [`TaskContext`] cannot be modified by other tasks, thus we can access it with raw pointer.
pub unsafe fn curr_ctx() -> *const TaskContext {
    &cpu().curr.as_ref().unwrap().inner().context
}

/// IDLE task context on this CPU.
pub fn idle_ctx() -> *const TaskContext {
    &cpu().idle_ctx as _
}

/// Kernel init task which will never be dropped.
pub static IDLE_TASK: Lazy<Arc<Task>> = Lazy::new(|| Arc::new(Task::idle().unwrap()));

/// Reclaim resources delegated to [`INIT_TASK`].
pub fn init_reclaim() {
    let mut childrens = IDLE_TASK.children.lock();
    childrens.clear();
}

/// IDLE task:
///
/// 1. Each cpu tries to acquire the lock of global task manager.
/// 2. Each cpu runs the task fetched from schedule queue.
/// 3. Handle the final state after a task finishes `do_yield` or `do_exit`.
/// 4. Reclaim resources handled by [`INIT_TASK`].
pub unsafe fn idle() -> ! {
    loop {
        init_reclaim();

        let mut task_manager = TASK_MANAGER.lock();
        log::trace!("idle");
        if let Some(task) = task_manager.fetch() {
            let next_ctx = {
                *task.state.lock() = TaskState::RUNNING;
                let ctx = &task.inner().context as *const TaskContext;
                ctx
            };
            log::trace!("Run {:?}", task);

            // Ownership moved to `current`.
            cpu().curr = Some(task);

            // Release the lock.
            drop(task_manager);
            log::trace!("{:#X?}", &*next_ctx);
            __switch(idle_ctx(), next_ctx);
            let curr = cpu().curr.take().unwrap();
            let state = curr.state();
            if state == TaskState::RUNNABLE {
                log::trace!("add {:?}", curr);
                TASK_MANAGER.lock().add(curr);
            } else if state == TaskState::ZOMBIE {
                handle_zombie(curr);
            } else {
                panic!("Unexpected state {:#?}", state);
            }
        } else {
            log::trace!("no task");
        }
    }
}

/// Current task exits. Run next task.
///
/// # Safety
///
/// Unsafe context switch will be called in this function.
pub unsafe fn do_exit(exit_code: isize) {
    let curr = cpu().curr.as_ref().unwrap();
    let _curr_ctx = {
        curr.inner().exit_code = exit_code;
        *curr.state.lock() = TaskState::ZOMBIE;
        &curr.inner().context as *const TaskContext
    };
    log::trace!("{:?} exited with code {}", curr, exit_code);

    __move_to_next(idle_ctx());
}

/// Current task suspends. Run next task.
///
/// # Safety
///
/// Unsafe context switch will be called in this function.
pub unsafe fn do_yield() {
    let curr = cpu().curr.as_ref().unwrap();
    let curr_ctx = {
        *curr.state.lock() = TaskState::RUNNABLE;
        &curr.inner().context as *const TaskContext
    };
    log::trace!("{:#?} suspended", curr);

    // Saves and restores CPU local variable, intena.
    let intena = CPUs[hart_id()].intena;
    __switch(curr_ctx, idle_ctx());
    CPUs[hart_id()].intena = intena;
}

// Handle zombie tasks.
/// 1. Children of current task will be delegated to [`INIT_TASK`].
/// 2. Current task may need to send a signal to its parent.
///
/// # DEAD LOCK
///
/// 1. Current task and its children are all in this function. The inner lock will be held.
/// 2. Current task acquires the lock of [`INIT_TASK`] but one of its children is waiting
/// for this lock.
/// 3. Current task cannot acquire the lock of this child, while this child cannot release
/// its lock until successfully acquiring the lock of [`INIT_TASK`].
///
/// So we need to acquire the locks in order:
///
/// ```
/// let mut child_inner = child.locked_inner();
/// let mut init_task_inner = INIT_TASK.locked_inner();
/// ```
///
/// to eliminate the dead lock when both current task and its child are trying acquire
/// the lock of [`INIT_TASK`]. If current task acquires the lock of its child, the child
/// must not be in this function, thus current task can successfully acquire the lock of
/// [`INIT_TASK`] with this child stuck at the beginning of this function. If current task
/// fails to acquire the lock of its child, it cannot acquire the lock of [`INIT_TASK`],
/// either, thus the child just in this function can acquire the lock of [`INIT_TASK`]
/// successfully and finally release both locks.
///
///
pub fn handle_zombie(task: Arc<Task>) {
    for child in task.children.lock().iter() {
        *child.parent.lock() = Some(Arc::downgrade(&IDLE_TASK));
        IDLE_TASK.children.lock().push_back(child.clone());
    }
    task.children.lock().clear();
    let orphan = task.parent.lock().is_none();

    if orphan {
        IDLE_TASK.children.lock().push_back(task);
    }
}