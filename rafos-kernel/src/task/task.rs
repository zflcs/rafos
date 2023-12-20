use core::sync::atomic::AtomicI32;

use super::*;

use kernel_sync::{SpinLock, SpinLockGuard};
use alloc::{vec::Vec, sync::{Arc, Weak}, string::{String, ToString}};
use crate::{mm::{MM, KERNEL_SPACE}, fs::FDManager, KernelResult, loader, trampoline::*};

pub struct Task {
    // immutable
    pub tid: TidHandle,
    pub pid: usize,
    pub trapframe_tracker: Option<TrapFrameTracker>,
    // mutable
    pub name: SpinLock<String>,
    pub state: SpinLock<TaskState>,
    pub context: SpinLock<TaskContext>,
    pub kstack: SpinLock<KernelStack>,
    pub mm: Arc<SpinLock<MM>>,
    pub parent: SpinLock<Option<Weak<Task>>>,
    pub children: SpinLock<Vec<Arc<Task>>>,
    pub exit_code: AtomicI32,
    pub fd_table: SpinLock<FDManager>,
}

impl Task {
    pub fn idle() -> KernelResult<Self> {
        Ok(Self {
            tid: TidHandle(IDLE_PID),
            pid: 0,
            name: SpinLock::new("idle".to_string()),
            state: SpinLock::new(TaskState::RUNNABLE),
            mm: Arc::new(SpinLock::new(MM::new(false).unwrap())),
            parent: SpinLock::new(None),
            children: SpinLock::new(Vec::new()),
            exit_code: AtomicI32::new(0),
            fd_table: SpinLock::new(FDManager::new()),
            trapframe_tracker: None,
            context: SpinLock::new(TaskContext::zero()),
            kstack: SpinLock::new(KernelStack::new()?),
        })
    }

    
    // pub fn new() -> Result<TaskRef, KernelError> {
    pub fn new(elf_data: &[u8]) -> KernelResult<Self> {
        let mut mm = MM::new(false)?;
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
            context: SpinLock::new(TaskContext::new(user_trap_return as usize, kstack_base)),
            kstack: SpinLock::new(kstack),
            mm: Arc::new(SpinLock::new(mm)),
            parent: SpinLock::new(None),
            children: SpinLock::new(Vec::new()),
            exit_code: AtomicI32::new(0),
            fd_table: SpinLock::new(FDManager::new()),
        };
        Ok(task)
    }

    pub fn trapframe(&self) -> &'static mut TrapFrame {
        TrapFrame::from(self.trapframe_tracker.as_ref().unwrap().0.start_address())
    }

    pub fn mm(&self) -> SpinLockGuard<MM> {
        self.mm.lock()
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

