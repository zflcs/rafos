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

use crate::{cpu::*, trampoline::{user_trap_return, user_trap_handler}, mm::{MM, KERNEL_SPACE}, loader, KernelError};

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


