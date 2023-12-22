use numeric_enum_macro::numeric_enum;
use macros::{GenSysMacro, GenSysTrait};
use super::*;

numeric_enum! {
    #[repr(usize)]
    #[derive(Debug, GenSysMacro, GenSysTrait)]
    pub enum SyscallProc {
        SchedYield = 24,
        Pause = 34,
        GetPid = 39,
        #[arguments(a0 = entry: usize, a1 = stack: usize, a2 = flags: usize, a3 = arg: *const usize, a4 = ptid: usize, a5 = tls: usize, a6 = ctid: usize)]
        Clone = 56,
        Fork = 57,
        Vfork = 58,
        #[arguments(a0 = filename: *const u8, a1 = argv: *const usize, a2 = envp: *const usize)]
        Execve = 59,
        #[arguments(a0 = exit_code: isize)]
        Exit = 60,
        #[arguments(a0 = pid: isize, a1 = wstatus: *mut isize, a2 = options: usize, a3 = rusage: usize)]
        Wait4 = 61,
    }
}

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct CloneFlags: usize {
        /// Signal Mask
        const CSIGNAL = 0x000000ff;
        const CLONE_NEWTIME = 0x00000080;
        /// Share memory space
        const CLONE_VM = 0x00000100;
        /// Share file system
        const CLONE_FS = 0x00000200;
        /// Share fd table
        const CLONE_FILES = 0x00000400;
        /// Share signal handler function
        const CLONE_SIGHAND = 0x00000800;
        /// Create a fd point to child task, it is used by `sys_pidfd_open`
        const CLONE_PIDFD = 0x00001000;
        /// Share sys_ptrace
        const CLONE_PTRACE = 0x00002000;
        /// The parent task will be blocked until the child task has exited
        const CLONE_VFORK = 0x00004000;
        /// The new task's pid will be the current task's parent
        const CLONE_PARENT = 0x00008000;
        /// Create a new thread, the pid is the same as the `CLONE_PARENT`, and the new task cannot be waited
        const CLONE_THREAD = 0x00010000;
        /// The child task will use new namespace
        const CLONE_NEWNS = 0x00020000;
        /// The semaphore of current task will be shared with child task. It is used by `sys_semop`
        const CLONE_SYSVSEM = 0x00040000;
        /// Set the `TLS`
        const CLONE_SETTLS = 0x00080000;
        /// Write the child task'tid in the target address
        const CLONE_PARENT_SETTID = 0x00100000;
        /// Clear the target address of child task, the address will be recorded, when the child task is exited, the futex in the address will be triggle
        const CLONE_CHILD_CLEARTID = 0x00200000;
        /// This flag will be ignored
        const CLONE_DETACHED = 0x00400000;
        /// Related with `sys_ptrace`
        const CLONE_UNTRACED = 0x00800000;
        /// Set the child task's tid in the target address
        const CLONE_CHILD_SETTID = 0x01000000;
        const CLONE_NEWCGROUP = 0x02000000;
        const CLONE_NEWUTS = 0x04000000;
        const CLONE_NEWIPC = 0x08000000;
        const CLONE_NEWUSER = 0x10000000;
        const CLONE_NEWPID = 0x20000000;
        const CLONE_NEWNET = 0x40000000;
        const CLONE_IO = 0x80000000;
    }
}


bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct WaitOptions: usize {
        /// Return immediately if no child has exited.
        const WNONHANG = 0x00000001;
        /// Also return if a child has stopped (but not traced via ptrace(2)).
        /// Status for traced children which have stopped is provided even if
        /// this option is not specified.
        const WUNTRACED = 0x00000002;
        /// Wait for children that have been stopped by a delivery of a signal.
        const WSTOPPED = 0x00000002;
        /// Wait for children that have terminated.
        const WEXITED = 0x00000004;
        /// Also return if a stopped child has been resumed by delivery of SIGCONT.
        const WCONTINUED = 0x00000008;
        /// Leave the child in a waitable state; a later wait call can be used to
        /// again retrieve the child status information.
        const WNOWAIT = 0x01000000;

        /* Linux specified */

        /// Do not wait for children of other threads in the same thread group.
        /// This was the default before Linux 2.4.
        const __WNOTHREAD = 0x20000000;
        /// Wait for all children, regardless of type ("clone" or "non-clone").
        const __WALL = 0x40000000;
        ///  Wait for "clone" children only.  If omitted, then wait for "non-clone"
        /// children only. (A "clone" child is one which delivers no signal, or a
        /// signal other than SIGCHLD to its parent upon termination.)  This option
        /// is ignored if __WALL is also specified.
        const __WCLONE = 0x80000000;
    }
}

