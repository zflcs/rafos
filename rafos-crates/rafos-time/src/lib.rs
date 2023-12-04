//!
//! 
//! 

#![no_std]
#![allow(unused)]

mod delay;
pub mod driver;
mod duration;
mod instant;




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