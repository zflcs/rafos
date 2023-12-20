//!
//! 
//! 

#![no_std]
#![allow(unused)]

mod delay;
pub mod driver;
mod duration;
mod instant;
mod config;
mod spec;

pub use config::*;
pub use spec::*;
pub use delay::{block_for, Delay};
pub use duration::Duration;
pub use instant::Instant;

/// Ticks per second of the global timebase.
///
/// This value is specified by the `tick-*` Cargo features, which
/// should be set by the time driver. Some drivers support a fixed tick rate, others
/// allow you to choose a tick rate with Cargo features of their own. You should not
/// set the `tick-*` features for embassy yourself as an end user.
#[cfg(feature = "board_qemu")]
pub const TICK_HZ: u64 = 12500000;
#[cfg(feature = "board_axu15eg")]
pub const TICK_HZ: u64 = 10000000;

const fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

pub(crate) const GCD_1K: u64 = gcd(TICK_HZ, 1_000);
pub(crate) const GCD_1M: u64 = gcd(TICK_HZ, 1_000_000);
pub(crate) const GCD_1G: u64 = gcd(TICK_HZ, 1_000_000_000);

use numeric_enum_macro::numeric_enum;
numeric_enum! {
    #[repr(usize)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
    #[allow(non_camel_case_types)]
    pub enum ClockType {
        REALTIME = 0,
        MONOTONIC = 1,
        PROCESS_CPUTIME_ID = 2,
        THREAD_CPUTIME_ID = 3,
    }
}

numeric_enum! {
    #[repr(usize)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum CPUClockType {
        PROF = 0,
        VIRT = 1,
        SCHED = 2,
        FD = 3,
    }
}

/// CPU clock identification.
///
/// Bit fields within a clock id:
/// - \[31:3\] hold either a pid or a file descriptor
/// - \[2\] indicates whether a cpu clock refers to a thread or a process
/// - \[1:0\] give [`CPUClockType`]
///
/// A clock id is invalid if bits 2, 1 and 0 are all set.
pub struct ClockID(i32);

impl ClockID {
    ///  Creates a new clock id.
    pub fn new(clock: usize) -> Self {
        Self(clock as i32)
    }

    /// Creates a new clock id from pid.
    pub fn new_proc(pid: usize, type_: CPUClockType) -> Self {
        Self((((!pid) << 3) | usize::from(type_)) as i32)
    }

    /// Creates a new clock id from tid.
    pub fn new_thread(tid: usize, type_: CPUClockType) -> Self {
        Self((((!tid) << 3) | usize::from(type_) | 4) as i32)
    }

    /// Gets pid from a clock id.
    pub fn get_pid(&self) -> usize {
        !(self.0 as usize >> 3)
    }

    /// Gets clock type.
    pub fn get_type(&self) -> CPUClockType {
        CPUClockType::try_from(self.0 as usize & 3).unwrap()
    }

    /// Returns whether it refers to a thread or a process.
    pub fn is_thread(&self) -> bool {
        (self.0 as usize & 4) != 0
    }

    /// Returns whether it refers to a process.
    pub fn is_proc(&self) -> bool {
        (self.0 as usize & 4) == 0
    }
}

/// System clock abstraction for different clocks.  
///
/// See more details in Linux `struct k_clock`.
pub trait Clock {
    /// Get the resolution of a global clock with the given identification.
    fn clock_getres(which: ClockID, tp: &mut TimeSpec) -> usize;

    /// Get a global clock with the given identificaton.
    fn clock_get(which: ClockID, tp: &mut TimeSpec) -> usize;
}