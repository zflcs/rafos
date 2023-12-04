#![no_std]
#![no_main]
#![feature(lang_items, alloc_error_handler)]
#![allow(internal_features, non_snake_case)]

mod heap;
extern crate alloc;
use core::future::Future;
use alloc::boxed::Box;
use executor::{Executor, TaskType, TaskRef, execute};
core::arch::global_asm!(include_str!("info.asm"));

static mut EXECUTOR: Executor = Executor::new();

pub mod lang_item {
    ///
    #[lang = "eh_personality"]
    #[no_mangle]
    pub fn rust_eh_personality() {}

    #[no_mangle]
    pub fn _Unwind_Resume() {}

    /// not_kernel panic
    #[panic_handler]
    fn panic(_info: &core::panic::PanicInfo) -> ! {
        unreachable!()
    }
}


#[no_mangle]
pub fn spawn(fut: Box<dyn Future<Output = i32> + 'static + Send + Sync>, priority: u32, task_type: TaskType) -> TaskRef {
    let executor = unsafe { &mut EXECUTOR };
    executor.spawn(fut, priority, task_type)
}

#[no_mangle]
pub fn poll_future() {
    let executor = unsafe { &mut EXECUTOR };
    while let Some(task_ref) = executor.fetch(0) {
        if let Some(task_ref) = execute(task_ref) {
            if (unsafe { &*task_ref.as_ptr() }).task_type == TaskType::KernelSche {
                executor.wake_task_from_ref(task_ref);
            }
        }
    }
}

#[no_mangle]
pub fn wake_task(task_ref: TaskRef) {
    executor::wake_task(task_ref);
}

extern "C" {
    fn put_str(ptr: *const u8, len: usize);
}

pub fn print(s: &str) {
    let byte = s.as_bytes();
    unsafe { put_str(byte.as_ptr(), byte.len()) };
}



