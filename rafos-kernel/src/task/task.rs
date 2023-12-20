use core::cell::SyncUnsafeCell;
use alloc::collections::LinkedList;
use super::*;

use kernel_sync::{SpinLock, SpinLockGuard};
use alloc::{sync::{Arc, Weak}, string::{String, ToString}};
use crate::{mm::{MM, KERNEL_SPACE}, fs::FDManager, KernelResult, loader, trampoline::*};

pub struct TaskInner {
    pub exit_code: i32,
    pub context: TaskContext,
    pub kstack: KernelStack,
    pub mm: Arc<SpinLock<MM>>,
    pub files: Arc<SpinLock<FDManager>>,
}

unsafe impl Send for TaskInner {}

pub struct Task {
    // immutable
    pub tid: TidHandle,
    pub pid: usize,
    pub trapframe_tracker: Option<TrapFrameTracker>,
    // mutable
    pub name: SpinLock<String>,
    pub state: SpinLock<TaskState>,
    pub parent: SpinLock<Option<Weak<Task>>>,
    pub children: SpinLock<LinkedList<Arc<Task>>>,
    pub inner: SyncUnsafeCell<TaskInner>,
}

impl Task {
    pub fn idle() -> KernelResult<Self> {
        Ok(Self {
            tid: TidHandle(IDLE_PID),
            pid: 0,
            name: SpinLock::new("idle".to_string()),
            state: SpinLock::new(TaskState::RUNNABLE),
            parent: SpinLock::new(None),
            children: SpinLock::new(LinkedList::new()),
            trapframe_tracker: None,
            inner: SyncUnsafeCell::new(TaskInner {
                exit_code: 0,
                context: TaskContext::zero(),
                kstack: KernelStack::new()?,
                mm: Arc::new(SpinLock::new(MM::new()?)),
                files: Arc::new(SpinLock::new(FDManager::new())),
            }),
        })
    }

    pub fn new(elf_data: &[u8]) -> KernelResult<Self> {
        let mut mm = MM::new()?;
        let sp = loader::from_elf(elf_data, &mut mm)?;
        log::debug!("{:?}", mm);
        let kstack = KernelStack::new()?;
        let kstack_base = kstack.base();
        let tid = TidHandle::new();
        let tid_num = tid.0;
        let trapframe_tracker = init_trapframe(&mut mm, tid_num)?;
        let trapframe_pa = trapframe_tracker.0.start_address();
        let trapframe = TrapFrame::from(trapframe_pa);
        *trapframe = TrapFrame::new(
            KERNEL_SPACE.lock().page_table.satp(), 
            kstack_base, 
            user_trap_handler as usize, 
            mm.entry.value(), 
            sp.value()
        );
        let task = Self {
            tid,
            pid: tid_num,
            trapframe_tracker: Some(trapframe_tracker),
            name: SpinLock::new("new".to_string()),
            state: SpinLock::new(TaskState::RUNNABLE),
            parent: SpinLock::new(None),
            children: SpinLock::new(LinkedList::new()),
            inner: SyncUnsafeCell::new(TaskInner {
                exit_code: 0,
                context: TaskContext::new(user_trap_return as usize, kstack_base),
                kstack,
                mm: Arc::new(SpinLock::new(mm)),
                files: Arc::new(SpinLock::new(FDManager::new())),
            }),
        };
        Ok(task)
    }

    pub fn trapframe(&self) -> &'static mut TrapFrame {
        TrapFrame::from(self.trapframe_tracker.as_ref().unwrap().0.start_address())
    }

    /// Mutable access to [`TaskInner`].
    pub fn inner(&self) -> &mut TaskInner {
        unsafe { &mut *self.inner.get() }
    }

    pub fn mm(&self) -> SpinLockGuard<MM> {
        self.inner().mm.lock()
    }

    /// Acquires inner lock to modify [`FDManager`].
    pub fn files(&self) -> SpinLockGuard<FDManager> {
        self.inner().files.lock()
    }

    pub fn state(&self) -> TaskState {
        *self.state.lock()
    }
}

impl Drop for Task {
    fn drop(&mut self) {
        log::trace!("Drop {:?}", self);
    }
}
use core::fmt;
impl fmt::Debug for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Task [{}] pid={} tid={}",
            self.name.lock(), self.pid, self.tid.0
        )
    }
}

