/// This mod define the task in kernel
/// - Process
/// - kernel coroutine


mod task;
mod id;
mod kstack;
mod trapframe;
mod context;

use core::sync::atomic::AtomicI32;

use alloc::{sync::Arc, string::ToString, vec::Vec};
use errno::Errno;
use kernel_sync::SpinLock;
use mmrv::*;
use syscall::SyscallResult;
pub use task::*;
use id::*;
use kstack::*;
pub use trapframe::*;
pub use context::*;

use crate::{cpu::*, trampoline::{user_trap_return, user_trap_handler}, fs::FDManager, write_user, mm::{MM, KERNEL_SPACE}, loader, KernelError, KernelResult};

bitflags::bitflags! {
    /// Five-state model:
    ///
    /// - **Running** or **Runnable** (R)
    /// - **Sleeping** states: **Interruptible** (S) and **Uninterruptible** (D).
    /// - **Stopped** (T)
    /// - **Zombie** (Z)
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct TaskState: u8 {
        /// The task is waiting in scheduler.
        const RUNNABLE = 1 << 0;

        /// The task takes up a CPU core to execute its code.
        const RUNNING = 1  << 1;

        /// A task will react to `SIGSTOP` or `SIGTSTP` signals and be brought back
        /// to running or runnable by `SIGCONT` signal.
        const STOPPED = 1 << 2;

        /// Task will only for resources to be available.
        const INTERRUPTIBLE = 1 << 3;

        /// Task will react to both signals and the availability of resources.
        const UNINTERRUPTIBLE = 1 << 4;

        /// When a task has completed its execution or is terminated, it will send the
        /// `SIGCHLD` signal to the parent task and go into the zombie state.
        const ZOMBIE = 1 << 5;

        /// Task dead
        const DEAD = 1 << 6;
    }
}



/// A helper for [`syscall_interface::SyscallProc::clone`]
pub fn do_fork() -> SyscallResult {
    let curr = cpu().curr.as_ref().unwrap();
    log::debug!("FORK {:?}", &curr);
    let mut mm = curr.mm().clone()?;
    log::debug!("{:?}", mm);
    // New kernel stack
    let kstack = KernelStack::new()?;
    let tid = TidHandle::new();
    let tid_num = tid.0;
    let kstack_base = kstack.base();
    
    // Init trapframe
    let trapframe_tracker = {
        let trapframe_tracker = init_trapframe(&mut mm, tid_num)?;
        let trapframe = TrapFrame::from(trapframe_tracker.0.start_address());
        trapframe.copy_from(curr.trapframe(), 0, kstack_base);
        trapframe_tracker
    };
    let new_task = Arc::new(Task {
        tid,
        pid: tid_num,
        trapframe_tracker: Some(trapframe_tracker),
        name: SpinLock::new(curr.name.lock().to_string()),
        state: SpinLock::new(TaskState::RUNNABLE),
        context: SpinLock::new(TaskContext::new(user_trap_return as usize, kstack_base)),
        kstack: SpinLock::new(kstack),
        mm: Arc::new(SpinLock::new(mm)),
        parent: SpinLock::new(Some(Arc::downgrade(&curr))),
        children: SpinLock::new(Vec::new()),
        exit_code: AtomicI32::new(0),
        fd_table: SpinLock::new(FDManager::new()),
    });

    curr.children.lock().push(new_task.clone());
    TASK_MANAGER.lock().add(new_task);
    Ok(tid_num)
}


/// A helper for [`syscall_interface::SyscallProc::wait4`].
pub fn do_wait(pid: isize, exit_code_ptr: usize) -> SyscallResult {
    loop {
        log::trace!("WAIT4 {}", pid);
        let curr = cpu().curr.as_ref().unwrap();
        if curr.children
            .lock()
            .iter()
            .find(|child| pid == -1 || pid as usize == child.tid.0)
            .is_none() {
                log::trace!("no child process");
            return Err(Errno::ESRCH);
        };
        let mut children = curr.children.lock();
        let pair = children
            .iter()
            .enumerate()
            .find(|(_, child)| 
                child.state() == TaskState::ZOMBIE && (pid == -1 || pid as usize == child.tid.0)
        );
        if let Some((idx, _)) = pair {
            let child = children.remove(idx);
            let pid = child.tid.0;
            let exit_code = child.exit_code.load(core::sync::atomic::Ordering::Relaxed);
            write_user!(curr.mm(), VirtAddr::from(exit_code_ptr), exit_code, i32)?;
            log::trace!("wait {}, exit_code_ptr {:#X} exit_code: {}", pid, exit_code_ptr, exit_code);
            return Ok(pid);
        } else {
            unsafe { 
                do_yield();
                log::trace!("here");
            }
        }
    }
}

/// A helper for [`syscall_interface::SyscallProc::execve`]
pub fn do_exec(elf_data: &[u8]) -> KernelResult {
    let curr = cpu().curr.as_ref().unwrap();
    log::trace!("EXEC {:?} ", &curr);

    // memory mappings are not preserved
    let mut mm = MM::new(false)?;
    let vsp = loader::from_elf(elf_data, &mut mm)?;

    // re-initialize kernel stack
    let kstack = KernelStack::new()?;
    let kstack_base = kstack.base();
    *curr.kstack.lock() = kstack;

    // re-initialize trapframe
    let trapframe = curr.trapframe();
    *trapframe = TrapFrame::new(
        KERNEL_SPACE.lock().page_table.satp(),
        kstack_base,
        user_trap_handler as usize,
        mm.entry.value(),
        vsp.into(),
    );
    mm.page_table
        .map(
            Page::from(VirtAddr::from(trapframe_base(curr.tid.0))),
            Frame::from(curr.trapframe_tracker.as_ref().unwrap().0.start_address()),
            PTEFlags::READABLE | PTEFlags::WRITABLE | PTEFlags::VALID,
        )
        .map_err(|_| KernelError::PageTableInvalid)?;
    log::debug!("{:?}", mm);
    *curr.mm() = mm;

    *curr.context.lock() = TaskContext::new(user_trap_return as usize, kstack_base);
    Ok(())
}