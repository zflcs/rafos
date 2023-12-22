use alloc::string::{ToString, String};
use numeric_enum_macro::numeric_enum;
use macros::{GenSysMacro, GenSysTrait};
use vfs::Stat;

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
        #[arguments(a0 = filename: *const u8, a1 = statbuf: *mut Stat)]
        Stat = 4,
        #[arguments(a0 = fd: usize, a1 = statbuf: *mut Stat)]
        FsStat = 5,
        #[arguments(a0 = filename: *const u8, a1 = statbuf: *mut Stat)]
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
        #[arguments(a0 = fd: usize, a1 = buf: *mut u8, a2 = count: usize)]
        Getdents = 78,
    }
}

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct FileMode: u32 {
        const FMODE_READ = 0x0;
        const FMODE_WRITE = 0x1;
        const FMODE_RDWR = 0x2;
        const FMODE_EXEC = 0x5; //read and execute
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

#[derive(Default, Debug, Clone)]
#[repr(C)]
pub struct StatTime {
    pub year: u32,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
}
bitflags::bitflags! {
    #[derive(Default)]
     pub struct InodeMode:u32{
        const S_SYMLINK = 0120000;
        const S_DIR = 0040000;
        const S_FILE = 0100000;
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct Dirent64 {
    /// ino is an inode number
    pub ino: u64,
    /// off is an offset to next linux_dirent
    pub off: i64,
    /// reclen is the length of this linux_dirent
    pub reclen: u16,
    /// type is the file type
    pub type_: DirentType,
    /// name is the filename (null-terminated)
    pub name: [u8; 0],
}

impl Dirent64 {
    pub fn get_name(&self) -> &str {
        unsafe {
            let name = self.name.as_ptr();
            let name = core::ffi::CStr::from_ptr(name as *const u8);
            name.to_str().unwrap()
        }
    }
    pub fn len(&self) -> usize {
        self.reclen as usize
    }
}

bitflags::bitflags! {
    #[derive(PartialEq, Eq, Debug, Clone, Copy)]
    pub struct DirentType:u8{
        const DT_UNKNOWN = 0;
        const DT_FIFO = 1;
        const DT_CHR = 2;
        const DT_DIR = 4;
        const DT_BLK = 6;
        const DT_REG = 8;
        const DT_LNK = 10;
        const DT_SOCK = 12;
        const DT_WHT = 14;
    }
}

impl ToString for DirentType {
    fn to_string(&self) -> String {
        match *self {
            DirentType::DT_UNKNOWN => "unknown".to_string(),
            DirentType::DT_FIFO => "fifo".to_string(),
            DirentType::DT_CHR => "char".to_string(),
            DirentType::DT_DIR => "dir".to_string(),
            DirentType::DT_BLK => "block".to_string(),
            DirentType::DT_REG => "regular".to_string(),
            DirentType::DT_LNK => "link".to_string(),
            DirentType::DT_SOCK => "sock".to_string(),
            DirentType::DT_WHT => "whiteout".to_string(),
            _ => "unknown".to_string(),
        }
    }
}