pub const SIGNONE: usize = 0;
pub const SIGHUP: usize = 1;
pub const SIGINT: usize = 2;
pub const SIGQUIT: usize = 3;
pub const SIGILL: usize = 4;
pub const SIGTRAP: usize = 5;
pub const SIGABRT: usize = 6;
pub const SIGBUS: usize = 7;
pub const SIGFPE: usize = 8;
pub const SIGKILL: usize = 9;
pub const SIGUSR1: usize = 10;
pub const SIGSEGV: usize = 11;
pub const SIGUSR2: usize = 12;
pub const SIGPIPE: usize = 13;
pub const SIGALRM: usize = 14;
pub const SIGTERM: usize = 15;
pub const SIGSTKFLT: usize = 16;
pub const SIGCHLD: usize = 17;
pub const SIGCONT: usize = 18;
pub const SIGSTOP: usize = 19;
pub const SIGTSTP: usize = 20;
pub const SIGTTIN: usize = 21;
pub const SIGTTOU: usize = 22;
pub const SIGURG: usize = 23;
pub const SIGXCPU: usize = 24;
pub const SIGXFSZ: usize = 25;
pub const SIGVTALRM: usize = 26;
pub const SIGPROF: usize = 27;
pub const SIGWINCH: usize = 28;
pub const SIGIO: usize = 29;
pub const SIGPWR: usize = 30;
pub const SIGSYS: usize = 31;
pub const SIGRTMIN: usize = 32;
pub const SIGRT1: usize = 33;
pub const SIGRT2: usize = 34;
pub const SIGRT3: usize = 35;
pub const SIGRT4: usize = 36;
pub const SIGRT5: usize = 37;
pub const SIGRT6: usize = 38;
pub const SIGRT7: usize = 39;
pub const SIGRT8: usize = 40;
pub const SIGRT9: usize = 41;
pub const SIGRT10: usize = 42;
pub const SIGRT11: usize = 43;
pub const SIGRT12: usize = 44;
pub const SIGRT13: usize = 45;
pub const SIGRT14: usize = 46;
pub const SIGRT15: usize = 47;
pub const SIGRT16: usize = 48;
pub const SIGRT17: usize = 49;
pub const SIGRT18: usize = 50;
pub const SIGRT19: usize = 51;
pub const SIGRT20: usize = 52;
pub const SIGRT21: usize = 53;
pub const SIGRT22: usize = 54;
pub const SIGRT23: usize = 55;
pub const SIGRT24: usize = 56;
pub const SIGRT25: usize = 57;
pub const SIGRT26: usize = 58;
pub const SIGRT27: usize = 59;
pub const SIGRT28: usize = 60;
pub const SIGRT29: usize = 61;
pub const SIGRT30: usize = 62;
pub const SIGRT31: usize = 63;


#[inline(always)]
pub const fn sigmask(sig: usize) -> u64 {
    1 << (sig as u64 - 1)
}

#[inline(always)]
pub const fn sigtest(sig: usize, mask: u64) -> bool {
    sigmask(sig) & mask != 0
}

#[inline(always)]
pub const fn sigvalid(sig: usize) -> bool {
    sig >= 1 && sig <= NSIG
}
pub const SIG_KERNEL_ONLY_MASK: u64 = sigmask(SIGKILL) | sigmask(SIGSTOP);

pub const SIG_KERNEL_STOP_MASK: u64 =
    sigmask(SIGSTOP) | sigmask(SIGTSTP) | sigmask(SIGTTIN) | sigmask(SIGTTOU);

pub const SIG_KERNEL_COREDUMP_MASK: u64 = sigmask(SIGQUIT)
    | sigmask(SIGILL)
    | sigmask(SIGTRAP)
    | sigmask(SIGABRT)
    | sigmask(SIGFPE)
    | sigmask(SIGSEGV)
    | sigmask(SIGBUS)
    | sigmask(SIGSYS)
    | sigmask(SIGXCPU)
    | sigmask(SIGXFSZ);

pub const SIG_KERNEL_IGNORE_MASK: u64 =
    sigmask(SIGCONT) | sigmask(SIGCHLD) | sigmask(SIGWINCH) | sigmask(SIGURG);

#[inline(always)]
pub fn sig_kernel_only(sig: usize) -> bool {
    sig == SIGKILL || sig == SIGSTOP
}

#[inline(always)]
pub fn sig_kernel_coredump(sig: usize) -> bool {
    sigtest(sig, SIG_KERNEL_COREDUMP_MASK)
}

#[inline(always)]
pub fn sig_kernel_ignore(sig: usize) -> bool {
    sigtest(sig, SIG_KERNEL_IGNORE_MASK)
}

#[inline(always)]
pub fn sig_kernel_stop(sig: usize) -> bool {
    sigtest(sig, SIG_KERNEL_STOP_MASK)
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

use alloc::collections::LinkedList;


/// The set of signals that are pending for delivery to the thread.
///
/// A signal may be blocked, which means that it will not be delivered until
/// it is later unblocked. Between the time when it is generated and when it
/// is delivered a signal is said to be pending.
///
/// Each thread in a process has an independent signal mask, which indicates
/// the set of signals that the thread is currently blocking. A thread can
/// manipulate its signal mask using pthread_sigmask(3). In a traditional
/// single-threaded application, sigprocmask(2) can be used to manipulate the
/// signal mask.
///
/// A child created via fork(2) inherits a copy of its parent's signal mask;
/// the signal mask is preserved across execve(2).
///
/// A signal may be process-directed or thread-directed. A process-directed
/// signal is one that is targeted at (and thus pending for) the process as a whole.
/// A signal may be process-directed because it was generated by the kernel for
/// reasons other than a hardware exception, or because it was sent using kill(2)
/// or sigqueue(3). A thread-directed signal is one that is targeted at a specific
/// thread. A signal may be thread-directed because it was generated as a consequence
/// of executing a specific machine-language instruction that triggered a hardware
/// exception (e.g., SIGSEGV for an invalid memory access, or SIGFPE for a math error),
/// or because it was targeted at a specific thread using interfaces such as tgkill(2)
/// or pthread_kill(3).
///
/// A process-directed signal may be delivered to any one of the threads that does
/// not currently have the signal blocked. If more than one of the threads has the
/// signal unblocked, then the kernel chooses an arbitrary thread to which to deliver
/// the signal.
///
/// A thread can obtain the set of signals that it currently has pending using
/// sigpending(2).  This set will consist of the union of the set of pending
/// process-directed signals and the set of signals pending for the calling thread.
///
/// A child created via fork(2) initially has an empty pending signal set; the pending
/// signal set is preserved across an execve(2).
#[derive(Debug, Default)]
pub struct SigPending {
    /// Pending signals will be added to this list.
    pub list: LinkedList<SigInfo>,

    /// A mask for current pending signals.
    pub mask: SigSet,
}

impl SigPending {
    /// Creates a new `SigPending`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Outstanding pending unblocked signals.
    pub fn is_pending(&self) -> bool {
        !self.mask.is_empty()
    }

    /// Adds a `SigInfo` to the pending list.
    ///
    /// Result is unpredictable if `signum` is out of range.
    pub fn add(&mut self, sig: SigInfo) {
        self.list.push_back(sig);
        self.mask.set(sig.signo as usize - 1);
    }

    /// Fetches a pending unblocked signal to handle.
    pub fn fetch(&mut self) -> Option<SigInfo> {
        // Finds a signal to handle.
        let mut siginfo = None;
        let mut first = 0;
        for (i, sig) in self.list.iter().enumerate() {
            if self.mask.get(sig.signo as usize - 1) {
                siginfo = Some(*sig);
                first = i;
                break;
            }
        }
        // Removes the signal from pending list.
        if siginfo.is_some() {
            self.list.remove(first);
            self.mask.unset(siginfo.unwrap().signo as usize - 1);
        }
        siginfo
    }
}


/// The `siginfo_t` data type is a structure with the following fields:
///
/// `si_signo`, `si_errno` and `si_code` are defined for all signals. (si_errno is
/// generally unused on Linux.) The rest of the struct may be a union, so that
/// one should read only the fields that are meaningful for the given signal.
#[derive(Debug, Clone, Copy)]
pub struct SigInfo {
    /// Signal number
    pub signo: i32,

    /// An error value
    pub errno: i32,

    /// Signal code
    pub code: i32,
}

/* SIGCHLD si_codes */
/// child has exited
pub const CLD_EXITED: usize = 1;
/// child was killed
pub const CLD_KILLED: usize = 2;
/// child terminated abnormally
pub const CLD_DUMPED: usize = 3;
/// traced child has trapped
pub const CLD_TRAPPED: usize = 4;
/// child has stopped
pub const CLD_STOPPED: usize = 5;
/// stopped child has continued
pub const CLD_CONTINUED: usize = 6;
pub const NSIGCHLD: usize = 6;