use numeric_enum_macro::numeric_enum;
use macros::{GenSysMacro, GenSysTrait};

numeric_enum! {
    #[repr(usize)]
    #[derive(Debug, GenSysMacro, GenSysTrait)]
    pub enum SyscallProc {
        SchedYield = 24,
        Pause = 34,
        GetPid = 39,
        #[arguments(a0 = flags: usize, a1 = newsp: usize, a2 = ptid: usize, a3 = ctid: usize)]
        Clone = 56,
        Fork = 57,
        Vfork = 58,
        #[arguments(a0 = filename: *const u8, a1 = argv: *const usize, a2 = envp: *const usize)]
        Execve = 59,
        #[arguments(a0 = exit_code: isize)]
        Exit = 60,
        #[arguments(a0 = pid: usize, a1 = stat_addr: *mut usize, a2 = options: usize, a3 = rusage: *mut usize)]
        Wait4 = 61,
        #[arguments(a0 = pid: usize, a1 = exit_code_ptr: *mut isize)]
        Waitpid = 1000,
        #[arguments(a0 = entry: usize, a1 = arg: *const usize)]
        ThreadCreate = 1001,
        #[arguments(a0 = tid: usize)]
        Waittid = 1002,

    }
}