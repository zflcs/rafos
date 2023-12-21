use numeric_enum_macro::numeric_enum;
use macros::{GenSysMacro, GenSysTrait};

pub const STDIN: usize = 0;
pub const STDOUT: usize = 1;

numeric_enum! {
    #[repr(usize)]
    #[derive(Debug, GenSysMacro, GenSysTrait)]
    pub enum SyscallFile {
        #[arguments(a0 = fd: usize, a1 = buf: *mut u8, a2 = count: usize)]
        Read = 0,
        #[arguments(a0 = fd: usize, a1 = buf: *const u8, a2 = count: usize)]
        Write = 1,
        #[arguments(a0 = filename: *const u8, a1 = flags: usize, a2 = mode: usize)]
        Open = 2,
        #[arguments(a0 = fd: usize)]
        Close = 3,
        #[arguments(a0 = filename: *const u8, a1 = statbuf: *mut StatBuf)]
        Stat = 4,
        #[arguments(a0 = fd: usize, a1 = statbuf: *mut StatBuf)]
        FsStat = 5,
        #[arguments(a0 = filename: *const u8, a1 = statbuf: *mut StatBuf)]
        LsStat = 6,
        // #[arguments()]
        // Poll = 7,
        #[arguments(a0 = fd: usize, a1 = offset: usize, a2 = origin: usize)]
        LsSeek = 8,
        #[arguments(a0 = fd: usize, a1 = buf: *mut u8, a2 = count: usize, a3 = offset: usize)]
        Pread64 = 17,
        #[arguments(a0 = fd: usize, a1 = buf: *const u8, a2 = count: usize, a3 = offset: usize)]
        Pwrite64 = 18,
        #[arguments(a0 = fd: usize, a1 = iov: *const IoVec, a2 = iovcnt: usize)]
        Readv = 19,
        #[arguments(a0 = fd: usize, a1 = iov: *const IoVec, a2 = iovcnt: usize)]
        Writev = 20,
        #[arguments(a0 = filename: *const u8, a1 = mod: usize)]
        Access = 21,
        #[arguments(a0 = fildes: usize)]
        Dup = 32,
        #[arguments(a0 = oldfd: usize, a1 = newfd: usize)]
        Dup2 = 33,
        #[arguments(a0 = out_fd: usize, a1 = in_fd: usize, a2 = offset: *const u8, a3 = count: usize)]
        SendFile = 40,
    }
}


/// Used in readv and writev.
///
/// Defined in sys/uio.h.
#[repr(C)]
pub struct IoVec {
    /// Starting address
    pub iov_base: usize,
    /// Number of bytes to transfer
    pub iov_len: usize,
}

#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct StatBuf {
    pub st_dev: u64,
    pub st_ino: u64,
    pub st_mode: u32,
    pub st_nlink: u32,
    pub st_uid: u32,
    pub st_gid: u32,
    pub st_rdev: u64,
    __pad: u64,
    pub st_size: u64,
    pub st_blksize: u32,
    __pad2: u32,
    pub st_blocks: u64,
    pub st_atime_sec: u64,
    pub st_atime_nsec: u64,
    pub st_mtime_sec: u64,
    pub st_mtime_nsec: u64,
    pub st_ctime_sec: u64,
    pub st_ctime_nsec: u64,
    unused: u64,
} //128
