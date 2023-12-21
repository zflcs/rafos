/// This mod define the task in kernel
/// - Process
/// - kernel coroutine


mod task;
mod id;
mod kstack;
mod trapframe;
mod context;
mod clone;
mod exit;

pub use clone::*;
pub use exit::*;

use core::cell::SyncUnsafeCell;

use alloc::{sync::Arc, string::String, collections::LinkedList, vec::Vec};
use kernel_sync::SpinLock;
use mmrv::*;
use syscall::SyscallResult;
pub use task::*;
use id::*;
use kstack::*;
pub use trapframe::*;
pub use context::*;

use crate::{cpu::*, trampoline::{user_trap_return, user_trap_handler}, fs::FDManager, mm::{MM, KERNEL_SPACE}, loader, KernelError};

bitflags::bitflags! {
    /// Five-state model:
    ///
    /// - **Running** or **Runnable** (R)
    /// - **Sleeping** states: **Interruptible** (S) and **Uninterruptible** (D).
    /// - **Stopped** (T)
    /// - **Zombie** (Z)
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct TaskState: u16 {
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
        /// Task wake and kill
        const WAKEKILL = 1 << 7;
        /// Task wake
        const WAKE = 1 << 8;
    }
}






/// A helper for [`syscall_interface::SyscallProc::clone`]
pub fn do_thread_create(entry:usize, arg:usize) -> SyscallResult {
    // let curr = cpu().curr.as_ref().unwrap();
    // log::trace!("FORK {:?}", &curr);
    // let mm = curr.inner().mm.clone();
    // let files = curr.inner().files.clone();
    // let fs_info = curr.fs_info.clone();
    // let pid = curr.pid;
    // let name = curr.inner().name.clone();
    // log::trace!("{:?}", mm);
    // // New kernel stack
    // let kstack = KernelStack::new()?;
    // let tid = TidHandle::new();
    // let tid_num = tid.0;
    // let kstack_base = kstack.base();
    // // Create a new user stack
    // let start_brk = mm.lock().start_brk;
    // let start_va = mm.lock().find_free_area(start_brk, USER_STACK_SIZE)?;
    // log::trace!("{:?}", start_va);
    // mm.lock().alloc_write_vma(
    //     None, 
    //     start_va, 
    //     start_va + USER_STACK_SIZE - ADDR_ALIGN, 
    //     VMFlags::READ | VMFlags::WRITE | VMFlags::USER
    // )?;
    // log::trace!("{:?}", mm);
    // // Init trapframe
    // let trapframe_tracker = {
    //     let mut mm = mm.lock();
    //     let trapframe_tracker = init_trapframe(&mut mm, tid_num)?;
    //     let trapframe = TrapFrame::from(trapframe_tracker.0.start_address());
    //     *trapframe = TrapFrame::new(
    //         KERNEL_SPACE.lock().page_table.satp(), 
    //         kstack_base, 
    //         user_trap_handler as usize, 
    //         entry, 
    //         (start_va + USER_STACK_SIZE - ADDR_ALIGN).into()
    //     );
    //     trapframe.set_a0(arg);
    //     trapframe_tracker
    // };
    // let new_task = Arc::new(Task {
    //     tid,
    //     pid,
    //     trapframe_tracker: Some(trapframe_tracker),
    //     state: SpinLock::new(TaskState::RUNNABLE),
    //     parent: SpinLock::new(Some(Arc::downgrade(curr))),
    //     children: SpinLock::new(LinkedList::new()),
    //     inner: SyncUnsafeCell::new(TaskInner {
    //         name,
    //         exit_code: 0,
    //         context:  TaskContext::new(user_trap_return as usize, kstack_base),
    //         kstack,
    //         mm,
    //         files,
    //     }),
    //     fs_info
    // });
    // TASK_MANAGER.lock().add(new_task.clone());
    // curr.children.lock().push_back(new_task);
    // Ok(tid_num)
    Ok(0)
}

