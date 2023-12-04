//! Coroutine Control Block structures for more control.
//!

use crate::Asyncc;

use super::{executor::Executor, waker};
use alloc::{boxed::Box, sync::Arc};
use core::{
    future::Future,
    pin::Pin,
    ptr::NonNull,
    sync::atomic::{AtomicU32, Ordering},
    task::{Context, Poll}
};
use crossbeam::atomic::AtomicCell;

/// 
#[repr(u32)]
pub enum TaskState {
    ///
    Ready = 1 << 0,
    /// 
    Running = 1 << 1,
    ///
    Pending = 1 << 2
}


/// The pointer of `Task`
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TaskRef {
    ptr: NonNull<Task>,
}

unsafe impl Send for TaskRef where &'static Task: Send {}
unsafe impl Sync for TaskRef where &'static Task: Sync {}

impl TaskRef {
    /// Safety: The pointer must have been obtained with `Task::as_ptr`
    pub(crate) unsafe fn from_ptr(ptr: *const Task) -> Self {
        Self {
            ptr: NonNull::new_unchecked(ptr as *mut Task),
        }
    }

    /// The returned pointer
    pub fn as_ptr(self) -> *const Task {
        self.ptr.as_ptr()
    }
}

///
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskType {
    ///
    KernelSche,
    ///
    Syscall,
    /// 
    AsyncSyscall,
    ///
    Other,
}

/// The `Task` is stored in heap by using `Arc`.
#[repr(C)]
pub struct Task {
    pub(crate) executor: &'static Executor,
    /// 
    pub state: AtomicU32,
    ///
    pub priority: AtomicU32,
    ///
    pub task_type: TaskType,
    /// 
    pub fut: AtomicCell<Box<dyn Future<Output = i32> + 'static + Send + Sync>>,
}

impl Task {
    /// Create a new Task, in not-spawned state.
    pub fn new(
        executor: &'static Executor,
        fut: Box<dyn Future<Output = i32> + 'static + Send + Sync>,
        priority: u32,
        task_type: TaskType,
    ) -> TaskRef {
        let task = Arc::new(Self {
            executor,
            state: AtomicU32::new(TaskState::Ready as _),
            priority: AtomicU32::new(priority),
            task_type,
            fut: AtomicCell::new(fut),
        });
        task.as_ref()
    }

    /// Update priority
    pub fn update_priority(&self, new_priority: u32) {
        self.priority.store(new_priority, Ordering::Relaxed);
    }

    ///
    pub fn as_ref(self: Arc<Self>) -> TaskRef {
        unsafe { TaskRef::from_ptr(Arc::into_raw(self)) }
    }

    /// 
    pub fn from_ref(task_ref: TaskRef) -> Arc<Self> {
        let raw_ptr = task_ref.as_ptr();
        unsafe { Arc::from_raw(raw_ptr) }
    }
}

/// Wake a task by `TaskRef`.
///
/// You can obtain a `TaskRef` from a `Waker` using [`task_from_waker`].
pub fn wake_task(task_ref: TaskRef) {
    unsafe {
        let task_ptr = task_ref.as_ptr();
        let executor = (*task_ptr).executor;
        executor.wake_task_from_ref(task_ref)
    }
}

/// 
#[inline(always)]
pub fn execute(task_ref: TaskRef) -> Option<TaskRef> {
    unsafe {
        let waker = waker::from_task(task_ref);
        let mut cx = Context::from_waker(&waker);
        let task = Task::from_ref(task_ref);
        task.state.store(TaskState::Running as _, Ordering::Relaxed);
        let fut = &mut *task.fut.as_ptr();
        let mut future = Pin::new_unchecked(fut.as_mut());
        match future.as_mut().poll(&mut cx) {
            Poll::Ready(_) => { 
                Asyncc::set_cause(crate::Cause::Finish);
                None },
            Poll::Pending => { 
                Asyncc::set_cause(crate::Cause::Await);
                task.state.store(TaskState::Pending as _, Ordering::Relaxed);
                Some(task.as_ref()) 
            },
        }
    }
}
