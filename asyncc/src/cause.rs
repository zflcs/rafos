
/// This mod define the causes of execution flow change


#[derive(Debug)]
#[repr(C)]
pub enum Cause {
    /// Coroutine finished
    Finish,
    /// Coroutine await
    Await,
    ///
    Intr(Interrupt),
    ///
    Exception(Exception)
}

impl From<u32> for Cause {
    #[inline]
    fn from(value: u32) -> Self {
        let code = value & !0xc0000000;
        let flag = (value >> 30) & 0x00000003;
        match flag {
            0 => Self::Finish,
            1 => Self::Await,
            2 => {
                let exception = Exception::from(code);
                Self::Exception(exception)
            },
            _ => {
                let intr = Interrupt::from(code);
                Self::Intr(intr)
            }
        }
    }
}

impl Into<u32> for Cause {
    fn into(self) -> u32 {
        match self {
            Self::Finish => 0,
            Self::Await => 1 << 30,
            Self::Exception(exception) => {
                (1 << 31) | exception as u32
            },
            Self::Intr(intr) => {
                (3 << 30) | intr as u32
            }
        }
    }
}

#[allow(missing_docs)]
#[derive(Debug)]
#[repr(C)]
pub enum Interrupt {
    UserSoft,
    SupervisorSoft,
    HypervisorSoft,
    MachineSoft,
    UserTimer,
    SupervisorTimer,
    HypervisorTimer,
    MachineTimer,
    UserExternal,
    SupervisorExternal,
    HypervisorExternal,
    MachineExternal,
    Undefine,
}

#[allow(missing_docs)]
#[derive(Debug)]
#[repr(C)]
pub enum Exception {
    InstMisaligned,
    InstAccessFault,
    IllegalInst,
    Breakpoint,
    LoadAddrMisaligned,
    LoadAccessFault,
    StoreAddrMisaligned,
    StoreAccessFault,
    UserEnvCall,
    SupervisorEnvCall,
    HypervisorEnvCall,
    MachineEnvCall,
    InstPageFault,
    LoadPageFault,
    Reserved14,
    StorePageFault,
    Undefine,
}


impl From<u32> for Interrupt {
    #[inline]
    fn from(nr: u32) -> Self {
        match nr {
            0 => Interrupt::UserSoft,
            1 => Interrupt::SupervisorSoft,
            2 => Interrupt::HypervisorSoft,
            3 => Interrupt::MachineSoft,
            4 => Interrupt::UserTimer,
            5 => Interrupt::SupervisorTimer,
            6 => Interrupt::HypervisorTimer,
            7 => Interrupt::MachineTimer,
            8 => Interrupt::UserExternal,
            9 => Interrupt::SupervisorExternal,
            10 => Interrupt::HypervisorExternal,
            11 => Interrupt::MachineExternal,
            _ => Interrupt::Undefine,
        }
    }
}

impl From<u32> for Exception {
    #[inline]
    fn from(nr: u32) -> Self {
        match nr {
            0 => Exception::InstMisaligned,
            1 => Exception::InstAccessFault,
            2 => Exception::IllegalInst,
            3 => Exception::Breakpoint,
            4 => Exception::LoadAddrMisaligned,
            5 => Exception::LoadAccessFault,
            6 => Exception::StoreAddrMisaligned,
            7 => Exception::StoreAccessFault,
            8 => Exception::UserEnvCall,
            9 => Exception::SupervisorEnvCall,
            10 => Exception::HypervisorEnvCall,
            11 => Exception::MachineEnvCall,
            12 => Exception::InstPageFault,
            13 => Exception::LoadPageFault,
            14 => Exception::Reserved14,
            15 => Exception::StorePageFault,
            _ => Exception::Undefine,
        }
    }
}


#[test]
fn cause_test() {
    let intr = 0xc0000000;
    for i in 0..12 {
        let cause_value = intr | i;
        let cause = Cause::from(cause_value);
        println!("{:?}", cause);
        let value: u32 = cause.into();
        println!("{:#X}", value);
        assert!(value == cause_value);
    }
    let exception = 0x80000000;
    for i in 0..16 {
        let cause_value = exception | i;
        let cause = Cause::from(cause_value);
        println!("{:?}", cause);
        let value: u32 = cause.into();
        println!("{:#X}", value);
        assert!(value == cause_value);
    }
}


