use numeric_enum_macro::numeric_enum;
use macros::{GenSysMacro, GenSysTrait};

numeric_enum! {
    #[repr(usize)]
    #[derive(Debug, GenSysMacro, GenSysTrait)]
    pub enum SyscallMM {
        #[arguments(a0 = addr: usize, a1 = len: usize, a2 = prot: usize, a3 = flags: usize, a4 = fd: usize, a5 = offset: usize)]
        Mmap = 9,
        #[arguments(a0 = start: usize, a1 = len: usize, a2 = prot: usize)]
        Mprotect = 10,
        #[arguments(a0 = start: usize, a1 = len: usize)]
        Munmap = 11,
        #[arguments(a0 = brk: usize)]
        Brk = 12,
        #[arguments(a0 = addr: usize, a1 = old_len: usize, a2 = new_len: usize, a3 = flags: usize, a4 = new_addr: usize)]
        Mremap = 25,
        #[arguments(a0 = start: usize, a1 = len: usize, a2 = flags: usize)]
        Msync = 26,
        #[arguments(a0 = start: usize, a1 = len: usize, a2 = vec: *const u8)]
        MinCore = 27,
        #[arguments(a0 = start: usize, a1 = len: usize, a2 = behavior: usize)]
        Madvise = 28,
        #[arguments(a0 = key: usize, a1 = size: usize, a2 = shmflg: usize)]
        Shmget = 29,
        #[arguments(a0 = shmid: usize, a1 = shmaddr: usize, a2: shmflg: usize)]
        Shmat = 30,
        #[arguments(a0 = shmid: usize, a1 = cmd: usize, a2: buf: *const usize)]
        Shmctl = 31,
    }
}