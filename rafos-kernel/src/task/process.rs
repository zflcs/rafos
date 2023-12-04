use core::{
    sync::atomic::AtomicI32, 
    future::Future,
    pin::Pin,
    task::{Poll, Context},
};

/// This mod define `Process`
/// 

use spin::{Lazy, Mutex};
use alloc::{vec::Vec, sync::{Arc, Weak}};
use crate::{mm::MM, fs::{File, FDManager}};

use super::{RecycleAllocator, TaskState};

pub struct Process {
    // immutable
    pub pid: PidHandle,
    // mutable
    pub state: Mutex<TaskState>,
    pub mm: Mutex<MM>,
    pub parent: Mutex<Option<Weak<Process>>>,
    pub children: Mutex<Vec<Arc<Process>>>,
    pub exit_code: AtomicI32,
    pub fd_table: Mutex<FDManager>,
}

impl Process {
    pub fn idle() -> Self {
        Self {
            pid: PidHandle(IDLE_PID),
            state: Mutex::new(TaskState::RUNNABLE),
            mm: Mutex::new(MM::new().unwrap()),
            parent: Mutex::new(None),
            children: Mutex::new(Vec::new()),
            exit_code: AtomicI32::new(0),
            fd_table: Mutex::new(FDManager::new())
        }
    }
}

impl Future for Process {
    type Output = i32;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        
        Poll::Pending
    }
}




pub static PID_ALLOCATOR: Lazy<Mutex<RecycleAllocator>> = Lazy::new(|| Mutex::new(RecycleAllocator::new()));


pub const IDLE_PID: usize = 0;

pub struct PidHandle(pub usize);

pub fn pid_alloc() -> PidHandle {
    PidHandle(PID_ALLOCATOR.lock().alloc())
}

impl Drop for PidHandle {
    fn drop(&mut self) {
        PID_ALLOCATOR.lock().dealloc(self.0);
    }
}
