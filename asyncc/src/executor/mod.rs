//! This mod defines the structures related to `async` runtime.
//!

mod executor;
mod queue;
mod task;
mod waker;

pub use executor::*;
pub use task::*;
pub use waker::*;

/// pub use priority::PRIO_LEVEL;
pub const PRIO_LEVEL: usize = 8;

///
mod priority {
    #[cfg(feature = "prio-level-4")]
    ///
    pub const PRIO_LEVEL: usize = 4;
    #[cfg(feature = "prio-level-8")]
    ///
    pub const PRIO_LEVEL: usize = 8;
    #[cfg(feature = "prio-level-16")]
    ///
    pub const PRIO_LEVEL: usize = 16;
}
