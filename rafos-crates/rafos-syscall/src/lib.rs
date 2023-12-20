//! generate the user syscall interface and the kernel syscall trait.

#![no_std]

extern crate macros;
mod stdio;
use macros::{GenSysMacro, GenSysTrait};
pub use stdio::*;
use numeric_enum_macro::numeric_enum;
use errno::Errno;
pub type SyscallResult = Result<usize, Errno>;

#[cfg(feature = "user")]
mod user;
#[cfg(feature = "user")]
pub use user::*;

numeric_enum! {
    #[repr(usize)]
    #[derive(Debug, GenSysMacro, GenSysTrait)]
    pub enum SyscallId {
        #[arguments(args = "fd")]
        Dup = 24,
        #[arguments(args = "fd, request, argp")]
        Ioctl = 29,
        #[arguments(args = "dirfd, path, mode")]
        Mkdir = 34,
        #[arguments(args = "dirfd, pathname, flags")]
        UnlinkAt = 35,
        #[arguments(args = "old_fd, old_name, new_fd, new_name, flag")]
        LinkAt = 37,
        #[arguments(args = "pathname, flags")]
        OpenAt = 56,
        #[arguments(args = "fd")]
        Close = 57,
        #[arguments(args = "pipe_ptr")]
        Pipe = 59,
        #[arguments(args = "fd, off, whence")]
        Lseek = 62,
        #[arguments(args = "fd, buf_ptr, buf_len")]
        Read = 63,
        #[arguments(args = "fd, buf_ptr, buf_len")]
        Write = 64,
        #[arguments(args = "fd, iov, iovcnt")]
        Readv = 65,
        #[arguments(args = "fd, iov, iovcnt")]
        Writev = 66,
        #[arguments(args = "fd, buf_ptr, buf_len, offset")]
        Pread = 67,
        #[arguments(args = "exit_code")]
        Exit = 93,
        #[arguments(args = "exit_code")]
        ExitGroup = 94,
        #[arguments(args = "req, rem")]
        NanoSleep = 101,
        #[arguments(args = "clockid, tp")]
        ClockGetTime = 113,
        Yield = 124,
        #[arguments(args = "tv")]
        GetTimeOfDay = 169,
        GetPid = 172,
        GetTid = 178,
        #[arguments(args = "addr")]
        Brk = 214,
        #[arguments(args = "start, len")]
        Munmap = 215,
        Fork = 220,
        #[arguments(args = "path_ptr, args_ptr")]
        Exec = 221,
        #[arguments(args = "start, len, prot, flags, fd, offset")]
        Mmap = 222,
        #[arguments(args = "start, len, prot")]
        Mprotect = 226,
        #[arguments(args = "pid, exit_code_ptr")]
        WaitPid = 260,
        #[arguments(args = "pid, resource, new_limit, old_limit")]
        PriLimit64 = 261,
        #[arguments(args = "path_ptr")]
        Spawn = 400,
        #[arguments(args = "buf_ptr, buf_len")]
        MailRead = 401,
        #[arguments(args = "pid, buf_ptr, buf_len")]
        MailWrite = 402,
        #[arguments(args = "tid")]
        InitUserTrap = 600,
        #[arguments(args = "pid, msg")]
        SendMsg = 601,
        #[arguments(args = "time_us, cid")]
        SetTimer = 602,
        #[arguments(args = "device_id")]
        ClaimExtInt = 603,
        #[arguments(args = "device_id, enable")]
        SetExtIntEnable = 604,
        #[arguments(args = "entry, arg")]
        ThreadCreate = 1000,
        #[arguments(args = "tid")]
        WaitTid = 1002,
        #[arguments(args = "blocking")]
        MutexCreate = 1010,
        #[arguments(args = "id")]
        MutexLock = 1011,
        #[arguments(args = "id")]
        MutexUnlock = 1012,
        #[arguments(args = "arg")]
        CondvarCreate = 1030,
        #[arguments(args = "condvar_id")]
        CondvarSignal = 1031,
        #[arguments(args = "condvar_id, mutex_id")]
        CondvarWait = 1032,
        #[arguments(args = "port")]
        Listen = 1200,
        #[arguments(args = "fd")]
        Accept = 1201,
    }
}
