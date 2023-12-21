use numeric_enum_macro::numeric_enum;
use macros::{GenSysMacro, GenSysTrait};


numeric_enum! {
    #[repr(usize)]
    #[derive(Debug, GenSysMacro, GenSysTrait)]
    pub enum SyscallSocket {
        #[arguments(a0 = family: usize, a1 = tp: usize, a2 = protocol: usize)]
        Socket = 41,
        #[arguments(a0 = fd: usize, a1 = uservaddr: *const usize, a2 = addrlen: usize)]
        Connect = 42,
        #[arguments(a0 = fd: usize, a1 = upeer_sockaddr: *const usize, a2 = upeer_addrlen: usize)]
        Accept = 43,
        

    }
}

