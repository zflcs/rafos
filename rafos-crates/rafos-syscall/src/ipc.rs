use numeric_enum_macro::numeric_enum;
use macros::{GenSysMacro, GenSysTrait};

use super::*;

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

