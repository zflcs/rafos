#![cfg_attr(not(feature = "std"), no_std)]
#![no_main]
#![feature(lang_items, alloc_error_handler)]
#![allow(internal_features)]

#[cfg(feature = "no_std")]
mod heap;
extern crate alloc;
use core::future::Future;
use alloc::boxed::Box;
use executor::{Executor, TaskType, TaskRef, execute};
use spin::Lazy;
core::arch::global_asm!(include_str!("info.asm"));

static EXECUTOR: Lazy<Executor> = Lazy::new(|| Executor::new());

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "no_std")]
pub mod lang_item {
    ///
    #[lang = "eh_personality"]
    #[no_mangle]
    pub fn rust_eh_personality() {}

    /// not_kernel panic
    #[panic_handler]
    fn panic(_info: &core::panic::PanicInfo) -> ! {
        unreachable!()
    }
}

extern "C" {
    #[allow(improper_ctypes)]
    fn main() -> Box<dyn Future<Output = i32> + 'static + Send + Sync>;
}

#[no_mangle]
pub unsafe fn entry() {
    let main_fut = main();
    spawn(main_fut, 0, TaskType::KernelSche);
    poll_future();
}

#[no_mangle]
pub fn spawn(
    fut: Box<dyn Future<Output = i32> + 'static + Send + Sync>, 
    priority: u32, 
    task_type: TaskType
) -> TaskRef {
    let executor = unsafe { &mut *(EXECUTOR.as_mut_ptr()) };
    executor.spawn(fut, priority, task_type)
}

#[no_mangle]
pub fn poll_future() {
    let executor = unsafe { &mut *(EXECUTOR.as_mut_ptr()) };
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
    #[cfg(feature = "std")]
    let start = std::time::Instant::now();
    executor::wake_task(task_ref);
    #[cfg(feature = "std")]
    std::println!("wake time {}ns", start.elapsed().as_nanos());
}
