use numeric_enum_macro::numeric_enum;
use macros::{GenSysMacro, GenSysTrait};



numeric_enum! {
    #[repr(usize)]
    #[derive(Debug, GenSysMacro, GenSysTrait)]
    pub enum SyscallIPC {
        #[arguments(a0 = sig: usize, a1 = act: *const SigAction, a2 = oact: *mut SigAction, a3 = sigsetsize: usize)]
        SigAction = 13,
        #[arguments(a0 = how: usize, a1 = nset: *const SigSet, a2 = oset: *mut SigSet, a3 = sigsetsize: usize)]
        SigProcMask = 14,
        SigReturn = 15,
        #[arguments(a0 = pipefd: *const usize, a1 = flags: usize)]
        Pipe = 22,
        #[arguments(a0 = pid: usize, a1 = sig: usize)]
        Kill = 62,
        
    }
}


bitflags::bitflags! {
    #[derive(Default, Clone, Copy, Debug)]
    pub struct SigActionFlags: usize {
        /// If signum is SIGCHLD, do not receive notification when child processes
        /// stop (i.e., when they receive one of SIGSTOP, SIGTSTP, SIGTTIN, or SIGTTOU)
        /// or resume (i.e., they receive SIGCONT) (see wait(2)). This flag is meaningful
        /// only when establishing a handler for SIGCHLD.
        const SA_NOCLDSTOP = 1 << 0;

        /// If signum is SIGCHLD, do not transform children into zombies when they terminate.
        /// See also waitpid(2). This flag is meaningful only when establishing a handler for
        /// SIGCHLD, or when setting that signal's disposition to SIG_DFL (not ignored).
        ///
        /// If the SA_NOCLDWAIT flag is set when establishing a handler for SIGCHLD, POSIX.1
        /// leaves it unspecified whether a SIGCHLD signal is generated when a child process
        /// terminates.  On Linux, a SIGCHLD signal is generated in this case; on some other
        /// implementations, it is not.
        const SA_NOCLDWAIT = 1 << 1;

        /// The signal handler takes three arguments, not one. In this case, sa_sigaction
        /// should be set instead of sa_handler.
        /// This flag is meaningful only when establishing a signal handler.
        const SA_SIGINFO = 1 << 2;

        /// Not intended for application use.  This flag is used by C libraries to indicate that
        /// the sa_restorer field contains the address of a "signal trampoline".
        /// See sigreturn(2) for more details.
        const SA_RESTORER = 1 << 26;

        /// Call the signal handler on an alternate signal stack provided by sigaltstack(2).
        /// If an alternate stack is not available, the default stack will be used.
        /// This flag is meaningful only when establishing a signal handler.
        const SA_ONSTACK = 1 << 27;

        /// Provide behavior compatible with BSD signal semantics by making certain system calls
        /// restartable across signals. This flag is meaningful only when establishing a signal
        /// handler. See signal(7) for a discussion of system call restarting.
        const SA_RESTART = 1 << 28;

        /// Do not add the signal to the thread's signal mask while the handler is executing,
        /// unless the signal is specified in act.sa_mask. Consequently, a further instance of
        /// the signal may be delivered to the thread while it is executing the handler. This
        /// flag is meaningful only when establishing a signal handler.
        ///
        /// SA_NOMASK is an obsolete, nonstandard synonym for this flag.
        const SA_NODEFER = 1 << 30;

        /// Restore the signal action to the default upon entry to the signal handler.
        /// This flag is meaningful only when establishing a signal handler.
        ///
        /// SA_ONESHOT is an obsolete, nonstandard synonym for this flag.
        const SA_RESETHAND = 1 << 31;
    }
}

/// For the default action.
pub const SIG_DFL: usize = 0;

/// Signal ignored.
pub const SIG_IGN: usize = 1;

/// The `sigaction` structure.
///
/// On some architectures a union is involved: do not assign to both
/// `sa_handler` and `sa_sigaction`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SigAction {
    /// `sa_handler` specifies the action to be associated with signum and is be
    /// one of the following:
    /// - `SIG_DFL`: for the default action.
    /// - `SIG_IGN`: to ignore this signal.
    /// - A pointer to a signal handler function. This function receives the signal
    /// number as its only argument.
    ///
    /// If `SA_SIGINFO` is specified in `sa_flags`, then `sa_sigaction` (instead of
    /// `sa_handler`) specifies the signal-handling function for `signum`. This function
    /// receives three arguments, as described below:
    /// - `sig`: The number of the signal that caused invocation of the handler.
    /// - `info`: A pointer to a `siginfo_t`, which is a structure containing further
    /// information about the signal, as described below.
    /// - `ucontext`: This is a pointer to a ucontext_t structure, cast to void *.  The
    /// structure pointed to by this field contains signal context information that was
    /// saved on the user-space stack by the kernel; for details, see sigreturn(2).
    /// Further information about the ucontext_t structure can be found in getcontext(3)
    /// and signal(7). Commonly, the handler function doesn't make any use of the third
    /// argument.
    pub handler: usize,

    /// Specifies a set of flags which modify the behavior of the signal.
    pub flags: SigActionFlags,

    /// The `sa_restorer` field is not intended for application use. (POSIX does not
    /// specify a sa_restorer field.)  Some further details of the purpose of this
    /// field can be found in sigreturn(2).
    pub restorer: usize,

    /// Specifies a mask of signals which should be blocked (i.e., added to the signal
    /// mask of the thread in which the signal handler is invoked) during execution of
    /// the signal handler. In addition, the signal which triggered the handler will be
    /// blocked, unless the SA_NODEFER flag is used.
    pub mask: SigSet,
}

impl Default for SigAction {
    fn default() -> Self {
        Self {
            handler: SIG_DFL,
            flags: SigActionFlags::empty(),
            restorer: 0,
            mask: SigSet::default(),
        }
    }
}

impl SigAction {
    /// Creates a new `SigAction`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns if the signal will be ignored.
    pub fn is_ignored(&self) -> bool {
        !self.flags.contains(SigActionFlags::SA_SIGINFO) && self.handler == SIG_IGN
    }

    /// Returns true if SIGINFO is set.
    pub fn is_siginfo(&self) -> bool {
        self.flags.contains(SigActionFlags::SA_SIGINFO)
    }
}

/// The possible effects an unblocked signal set to SIG_DFL can have are:
pub enum SigActionDefault {
    /// Default action is to terminate the process.
    Term,

    /// Default action is to ignore the signal.
    Ign,

    /// Default action is to terminate the process and dump core.
    Core,

    /// Default action is to stop the process.
    Stop,

    /// Default action is to continue the process if it is currently stopped.
    Cont,
}

pub const NSIG: usize = 64;

pub type SigActions = [SigAction; NSIG];


///
#[derive(Debug, Default, Clone, Copy)]
pub struct SigSet(u64);

impl SigSet {
    /// Creates a new `SigSet`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Clears all bits set.
    pub fn clear(&mut self) {
        self.0 = 0;
    }

    /// Returns true if no bit is set.
    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }

    /// Returns true if the bit set.
    pub fn get(&self, kth: usize) -> bool {
        ((self.0 >> kth) & 1) == 1
    }

    /// Sets the bit.
    pub fn set(&mut self, kth: usize) {
        self.0 |= 1 << kth;
    }

    /// Sets bits in mask
    pub fn set_mask(&mut self, mask: u64) {
        self.0 |= mask;
    }

    /// Unsets the bit.
    pub fn unset(&mut self, kth: usize) {
        self.0 &= !(1 << kth);
    }

    /// Unsets bits in mask
    pub fn unset_mask(&mut self, mask: u64) {
        self.0 &= !mask;
    }

    /// Gets union.
    pub fn union(&mut self, other: &SigSet) {
        self.0 |= other.0;
    }

    /// Gets intersection.
    pub fn intersection(&mut self, other: &SigSet) {
        self.0 &= other.0;
    }

    /// Gets difference.
    pub fn difference(&mut self, other: &SigSet) {
        self.0 &= !other.0;
    }
}

impl From<u64> for SigSet {
    fn from(value: u64) -> Self {
        Self(value)
    }
}