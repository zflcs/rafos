use numeric_enum_macro::numeric_enum;
use macros::{GenSysMacro, GenSysTrait};

numeric_enum! {
    #[repr(usize)]
    #[derive(Debug, GenSysMacro, GenSysTrait)]
    pub enum SyscallIO {
        #[arguments(a0 = fd: usize, a1 = request: usize, a2 = argp: *const usize)]
        Ioctl = 16,
    }
}