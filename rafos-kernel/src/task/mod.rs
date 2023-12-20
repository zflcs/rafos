/// This mod define the task in kernel
/// - Process
/// - kernel coroutine


mod task;
mod id;
mod kstack;
mod trapframe;
mod context;

use core::cell::SyncUnsafeCell;

use alloc::{sync::Arc, string::ToString, collections::LinkedList};
use config::{USER_STACK_SIZE, USER_STACK_BASE, ADDR_ALIGN};
use errno::Errno;
use kernel_sync::SpinLock;
use mmrv::*;
use syscall::SyscallResult;
pub use task::*;
use id::*;
use kstack::*;
pub use trapframe::*;
pub use context::*;

use crate::{cpu::*, trampoline::{user_trap_return, user_trap_handler}, fs::FDManager, write_user, mm::{MM, KERNEL_SPACE, VMFlags}, loader, KernelError, KernelResult};

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
    log::trace!("FORK {:?}", &curr);
    let mm = Arc::new(SpinLock::new(curr.mm().fork()?));
    log::trace!("{:?}", mm);
    // New kernel stack
    let kstack = KernelStack::new()?;
    let tid = TidHandle::new();
    let tid_num = tid.0;
    let kstack_base = kstack.base();
    
    // Init trapframe
    let trapframe_tracker = {
        let mut mm = mm.lock();
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
        parent: SpinLock::new(Some(Arc::downgrade(&curr))),
        children: SpinLock::new(LinkedList::new()),
        inner: SyncUnsafeCell::new(TaskInner {
            exit_code: 0,
            context:  TaskContext::new(user_trap_return as usize, kstack_base),
            kstack,
            mm,
            files: Arc::new(SpinLock::new(FDManager::new())),
        })
    });
    TASK_MANAGER.lock().add(new_task.clone());
    curr.children.lock().push_back(new_task);
    Ok(tid_num)
}

/// A helper for [`syscall_interface::SyscallProc::clone`]
pub fn do_thread_create(entry:usize, arg:usize) -> SyscallResult {
    let curr = cpu().curr.as_ref().unwrap();
    log::trace!("FORK {:?}", &curr);
    let mm = curr.inner().mm.clone();
    let files = curr.inner().files.clone();
    let pid = curr.pid;
    log::debug!("{:?}", mm);
    // New kernel stack
    let kstack = KernelStack::new()?;
    let tid = TidHandle::new();
    let tid_num = tid.0;
    let kstack_base = kstack.base();
    // Create a new user stack
    let start_brk = mm.lock().start_brk;
    let start_va = mm.lock().find_free_area(start_brk, USER_STACK_SIZE)?;
    log::trace!("{:?}", start_va);
    mm.lock().alloc_write_vma(
        None, 
        start_va, 
        start_va + USER_STACK_SIZE - ADDR_ALIGN, 
        VMFlags::READ | VMFlags::WRITE | VMFlags::USER
    )?;
    log::trace!("{:?}", mm);
    // Init trapframe
    let trapframe_tracker = {
        let mut mm = mm.lock();
        let trapframe_tracker = init_trapframe(&mut mm, tid_num)?;
        let trapframe = TrapFrame::from(trapframe_tracker.0.start_address());
        *trapframe = TrapFrame::new(
            KERNEL_SPACE.lock().page_table.satp(), 
            kstack_base, 
            user_trap_handler as usize, 
            entry, 
            (start_va + USER_STACK_SIZE - ADDR_ALIGN).into()
        );
        trapframe_tracker
    };
    let new_task = Arc::new(Task {
        tid,
        pid,
        trapframe_tracker: Some(trapframe_tracker),
        name: SpinLock::new(curr.name.lock().to_string()),
        state: SpinLock::new(TaskState::RUNNABLE),
        parent: SpinLock::new(Some(Arc::downgrade(curr))),
        children: SpinLock::new(LinkedList::new()),
        inner: SyncUnsafeCell::new(TaskInner {
            exit_code: 0,
            context:  TaskContext::new(user_trap_return as usize, kstack_base),
            kstack,
            mm,
            files,
        })
    });
    TASK_MANAGER.lock().add(new_task.clone());
    curr.children.lock().push_back(new_task);
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
            let exit_code = child.inner().exit_code;
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

///
pub fn do_wait_tid(tid: usize) -> SyscallResult {
    loop {
        log::trace!("wait_tid {}", tid);
        let curr = cpu().curr.as_ref().unwrap();
        let pid = curr.pid;
        if curr.children
            .lock()
            .iter()
            .find(|child| tid == child.tid.0 && pid == child.pid)
            .is_none() {
                log::trace!("no such process");
            return Err(Errno::ESRCH);
        };
        let mut children = curr.children.lock();
        let pair = children
            .iter()
            .enumerate()
            .find(|(_, child)| 
                child.state() == TaskState::ZOMBIE && (tid == child.tid.0 && pid == child.pid)
        );
        if let Some((idx, _)) = pair {
            let child = children.remove(idx);
            let exit_code = child.inner().exit_code;
            return Ok(exit_code as _);
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
    let mut mm = MM::new()?;
    let vsp = loader::from_elf(elf_data, &mut mm)?;

    // re-initialize kernel stack
    let kstack = KernelStack::new()?;
    let kstack_base = kstack.base();
    curr.inner().kstack = kstack;

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
    log::trace!("{:?}", mm);

    curr.inner().mm = Arc::new(SpinLock::new(mm));
    curr.inner().context = TaskContext::new(user_trap_return as usize, kstack_base);
    Ok(())
}