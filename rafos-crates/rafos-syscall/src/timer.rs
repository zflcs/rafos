use numeric_enum_macro::numeric_enum;
use macros::{GenSysMacro, GenSysTrait};
use time::{ITimerVal, TimeSpec};

numeric_enum! {
    #[repr(usize)]
    #[derive(Debug, GenSysMacro, GenSysTrait)]
    pub enum SyscallTimer {
        #[arguments(a0 = rqtp: *const TimeSpec, a1 = rmtp: *mut TimeSpec)]
        NanoSleep = 35,
        #[arguments(a0 = which: usize, a1 = value: *mut ITimerVal)]
        Getitimer = 36,
        #[arguments(a0 = seconds: usize)]
        Alarm = 37,
        #[arguments(a0 = which: usize, a1 = value: *mut ITimerVal, a2 = ovalue: *mut ITimerVal)]
        Setitimer = 38,
        
    }
}