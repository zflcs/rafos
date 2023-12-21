use core::cell::SyncUnsafeCell;
use alloc::collections::LinkedList;
use syscall::{SigActions, SigSet, SigPending, SigAction, NSIG, SIGNONE};
use super::*;

use kernel_sync::{SpinLock, SpinLockGuard};
use alloc::{sync::{Arc, Weak}, string::String};
use crate::{mm::{MM, KERNEL_SPACE}, fs::{FDManager, FSInfo}, KernelResult, loader, trampoline::*};

pub struct TaskInner {
    pub name: String,
    pub exit_code: isize,
    pub context: TaskContext,
    pub kstack: KernelStack,
    pub mm: Arc<SpinLock<MM>>,
    pub files: Arc<SpinLock<FDManager>>,
    pub set_child_tid: usize,
    pub clear_child_tid: usize,
    pub sig_pending: SigPending,
    pub sig_blocked: SigSet,
}

unsafe impl Send for TaskInner {}

pub struct Task {
    /* immutable */ 
    /// The only tid
    pub tid: TidHandle,
    /// The tid of the task group leader
    pub pid: usize,
    /// The trapfame of the `Task`. If the task is kernel task, this field will be `None`.
    pub trapframe_tracker: Option<TrapFrameTracker>,
    pub exit_signal: usize,
    
    /* mutable but be not protected by lock */
    pub inner: SyncUnsafeCell<TaskInner>,
    
    /* mutable and be protected by lock */
    pub state: SpinLock<TaskState>,
    pub parent: SpinLock<Option<Weak<Task>>>,
    pub children: SpinLock<LinkedList<Arc<Task>>>,
    
    /* mutable and can share with other task */
    pub fs_info: Arc<SpinLock<FSInfo>>,
    pub sig_actions: Arc<SpinLock<SigActions>>,
}

impl Task {
    pub fn idle() -> KernelResult<Self> {
        Ok(Self {
            tid: TidHandle(IDLE_PID),
            pid: 0,
            state: SpinLock::new(TaskState::RUNNABLE),
            parent: SpinLock::new(None),
            children: SpinLock::new(LinkedList::new()),
            exit_signal: SIGNONE,
            trapframe_tracker: None,
            inner: SyncUnsafeCell::new(TaskInner {
                name: String::from("idle"),
                exit_code: 0,
                context: TaskContext::zero(),
                kstack: KernelStack::new()?,
                mm: Arc::new(SpinLock::new(MM::new()?)),
                files: Arc::new(SpinLock::new(FDManager::new())),
                set_child_tid: 0,
                clear_child_tid: 0,
                sig_pending: SigPending::new(),
                sig_blocked: SigSet::new(),
            }),
            fs_info: Arc::new(SpinLock::new(FSInfo {
                umask: 0,
                cwd: String::from("/"),
                root: String::from("/"),
            })),
            sig_actions: Arc::new(SpinLock::new([SigAction::default(); NSIG])),
        })
    }

    pub fn new(dir: String, elf_data: &[u8], args: Vec<String>) -> KernelResult<Self> {
        let mut mm = MM::new()?;
        let args_len = args.len();
        let vsp = loader::from_elf(elf_data, &mut mm, args)?;
        log::trace!("{:?}", mm);
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
            vsp.value()
        );
        trapframe.set_a0(args_len);
        trapframe.set_a1(vsp.into());
        let task = Self {
            tid,
            pid: tid_num,
            exit_signal: SIGNONE,
            trapframe_tracker: Some(trapframe_tracker),
            state: SpinLock::new(TaskState::RUNNABLE),
            parent: SpinLock::new(None),
            children: SpinLock::new(LinkedList::new()),
            inner: SyncUnsafeCell::new(TaskInner {
                name: dir.clone(),
                exit_code: 0,
                context: TaskContext::new(user_trap_return as usize, kstack_base),
                kstack,
                mm: Arc::new(SpinLock::new(mm)),
                files: Arc::new(SpinLock::new(FDManager::new())),
                set_child_tid: 0,
                clear_child_tid: 0,
                sig_pending: SigPending::new(),
                sig_blocked: SigSet::new(),
            }),
            fs_info: Arc::new(SpinLock::new(FSInfo {
                umask: 0,
                cwd: dir,
                root: String::from("/"),
            })),
            sig_actions: Arc::new(SpinLock::new([SigAction::default(); NSIG])),
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
            self.inner().name, self.pid, self.tid.0
        )
    }
}

