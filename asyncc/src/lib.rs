//!
//! 
#![cfg_attr(not(test), no_std)]
#![feature(allocator_api)]
#![deny(missing_docs)]

extern crate alloc;

mod executor;
mod cause;
mod queue;

pub use executor::*;
pub use cause::*;
pub use queue::*;

/// 
pub struct Asyncc {
    ///
    base_address: usize
}

impl Default for Asyncc {
    fn default() -> Self {
        Self { base_address: 0x6004_0000 }
    }
}

///
pub fn get_asyncc() -> &'static Asyncc {
    let asyncc_ptr = 0x6004_0000 as *const Asyncc;
    unsafe { &*asyncc_ptr }
}

impl Asyncc {

    ///
    pub fn new(base_address: usize) -> Self {
        Self { base_address }
    }

    ///
    #[inline]
    fn hardware(&self) -> &asyncc_pac::asyncc::RegisterBlock {
        unsafe { &*(self.base_address as *const _) }
    }

    ///
    pub fn get_executor(&self) -> &'static Executor {
        let hardware = self.hardware();
        let executor_ptr = hardware.eptr.read().bits() as *const usize as *const Executor;
        unsafe { &*executor_ptr }
    }

    /// init
    pub fn reset(&self, executor: *const Executor) {
        self.hardware().eptr.write(|w| unsafe { w.bits(executor as *const u32 as _) });
    }

    /// 
    pub fn is_finished(&self) -> bool {
        self.hardware().status.read().mode().is_finish()
    }

    /// 
    pub fn is_await(&self) -> bool {
        self.hardware().status.read().mode().is_await()
    }

    /// 
    pub fn is_exception(&self) -> bool {
        self.hardware().status.read().mode().is_exception()
    }

    /// 
    pub fn is_interrupt(&self) -> bool {
        self.hardware().status.read().mode().is_interrupt()
    }


    /// 
    pub fn cause(&self) -> Cause {
        self.hardware().status.read().bits().into()
    }

    /// 
    pub fn set_cause(&self, cause: Cause) {
        self.hardware().status.write(|w| unsafe { w.bits(cause.into()) });
    }

    /// 
    pub fn set_msgbuf(&self, msgbuf: usize) {
        self.hardware().msgbuf.write(|w| unsafe { w.bits(msgbuf as _) });
    }

    ///
    pub fn get_msgqueue(&self) -> &'static MsgQueue {
        let queue_ptr = self.hardware().msgbuf.read().bits() as *const usize as *const MsgQueue;
        unsafe { &*queue_ptr }
    }

    ///
    pub fn set_curr(&self, task_ref: TaskRef) {
        self.hardware().curc.write(|w| unsafe { w.bits(task_ref.as_ptr() as *const u32 as _) });
    }

    ///
    pub fn get_curr(&self) -> TaskRef {
        let task_raw_ptr = self.hardware().curc.read().bits() as *const u32 as *const Task;
        unsafe { TaskRef::from_ptr(task_raw_ptr) }
    }




}