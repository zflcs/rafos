//!
//! 
#![cfg_attr(not(test), no_std)]
#![deny(missing_docs)]

extern crate alloc;

mod executor;
mod cause;
mod queue;
mod args;

pub use executor::*;
pub use cause::*;
pub use queue::*;
pub use args::*;

use alloc::boxed::Box;
use core::future::Future;

use config::ASYNCC_ADDR;

/// 
#[derive(Debug)]
pub struct Asyncc;


impl Asyncc {

    ///
    #[inline]
    fn hardware() -> &'static asyncc_pac::asyncc::RegisterBlock {
        unsafe { &*(ASYNCC_ADDR as *const _) }
    }

    ///
    #[inline(always)]
    pub fn get_executor() -> &'static mut Executor {
        
        let hardware = Self::hardware();
        let executor_ptr = hardware.eptr().read().bits() as *const usize as *mut Executor;
        unsafe { &mut *executor_ptr }
    }

    ///
    pub fn spawn(fut: Box<dyn Future<Output = i32> + 'static + Send + Sync>, priority: u32, task_type: TaskType) -> TaskRef {
        let executor = Self::get_executor();
        executor.spawn(fut, priority, task_type)
    }

    ///
    pub fn spawn_task(task_ref: TaskRef) {
        let executor = Self::get_executor();
        executor.wake_task_from_ref(task_ref);
    }

    /// init
    pub fn reset(executor: *const Executor) {
        Self::hardware().eptr().write(|w| unsafe { w.bits(executor as *const u32 as _) });
    }

    /// 
    pub fn is_finished() -> bool {
        Self::hardware().status().read().mode().is_finish()
    }

    /// 
    pub fn is_await() -> bool {
        Self::hardware().status().read().mode().is_await()
    }

    /// 
    pub fn is_exception() -> bool {
        Self::hardware().status().read().mode().is_exception()
    }

    /// 
    pub fn is_interrupt() -> bool {
        Self::hardware().status().read().mode().is_interrupt()
    }


    /// 
    #[inline(always)]
    pub fn cause() -> Cause {
        Self::hardware().status().read().bits().into()
    }

    /// 
    pub fn set_cause(cause: Cause) {
        Self::hardware().status().write(|w| unsafe { w.bits(cause.into()) });
    }

    /// 
    pub fn set_msgbuf(msgbuf: usize) {
        Self::hardware().msgbuf().write(|w| unsafe { w.bits(msgbuf as _) });
    }

    ///
    pub fn get_msgqueue() -> &'static MsgQueue {
        let queue_ptr = Self::hardware().msgbuf().read().bits() as *const usize as *const MsgQueue;
        unsafe { &*queue_ptr }
    }

    ///
    pub fn set_curr(task_ref: Option<TaskRef>) {
        let curc = Self::hardware().curc();
        if let Some(task_ref) = task_ref {
            curc.write(|w| unsafe { w.bits(task_ref.as_ptr() as *const u32 as _) });
        } else {
            curc.write(|w| unsafe { w.bits(0) });
        }
    }

    ///
    pub fn get_curr() -> Option<TaskRef> {
        let task_raw_ptr = Self::hardware().curc().read().bits();
        if task_raw_ptr == 0 {
            return None;
        }
        unsafe { Some(TaskRef::from_ptr(task_raw_ptr as *const u32 as *const Task)) }
    }

    ///
    pub fn set_args(a0: usize) {
        let hardware = Self::hardware();
        hardware.a0_lsb().write(|w| unsafe { w.bits(a0 as u32 & 0xFFFFFFFF) });
        hardware.a0_msb().write(|w| unsafe { w.bits((a0 >> 32) as u32 & 0xFFFFFFFF) });
    }

    ///
    pub fn set_args2(a0: usize, a1: usize) {
        let hardware = Self::hardware();
        hardware.a0_lsb().write(|w| unsafe { w.bits(a0 as u32 & 0xFFFFFFFF) });
        hardware.a0_msb().write(|w| unsafe { w.bits((a0 >> 32) as u32 & 0xFFFFFFFF) });
        hardware.a1_lsb().write(|w| unsafe { w.bits(a1 as u32 & 0xFFFFFFFF) });
        hardware.a1_msb().write(|w| unsafe { w.bits((a1 >> 32) as u32 & 0xFFFFFFFF) });
    }

    ///
    pub fn get_args() -> Args {
        let mut _a0 = 0usize;
        let hardware = Self::hardware();
        _a0 = hardware.a0_lsb().read().bits() as _;
        _a0 |= (hardware.a0_msb().read().bits() as usize) << 32;
        Args { a: [_a0, 0, 0, 0, 0, 0, 0, 0] }
    }

    ///
    pub fn get_args2() -> Args {
        let mut _a0 = 0usize;
        let mut _a1 = 0usize;
        let hardware = Self::hardware();
        _a0 = hardware.a0_lsb().read().bits() as _;
        _a0 |= (hardware.a0_msb().read().bits() as usize) << 32;
        _a1 = hardware.a1_lsb().read().bits() as _;
        _a1 |= (hardware.a1_msb().read().bits() as usize) << 32;
        Args { a: [_a0, _a1, 0, 0, 0, 0, 0, 0] }
    }




}