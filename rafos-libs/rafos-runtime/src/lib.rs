
#![no_std]
#![no_main]
#![feature(lang_items, alloc_error_handler)]
#![allow(internal_features)]
#![no_builtins]

mod heap;
extern crate alloc;
use core::future::Future;
use alloc::boxed::Box;
use executor::{Executor, Task, TaskType};
use spin::Lazy;
core::arch::global_asm!(include_str!("info.asm"));

static EXECUTOR: Lazy<Executor> = Lazy::new(|| Executor::new());


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
pub fn spawn(fut: Box<dyn Future<Output = i32> + 'static + Send + Sync>, priority: u32, task_type: TaskType) {
    let task = Task::new(fut, priority, task_type);
    let executor = unsafe { &mut *(EXECUTOR.as_mut_ptr()) };
    executor.spawn(task);
}

#[no_mangle]
pub fn poll_future() {
    let executor = unsafe { &mut *(EXECUTOR.as_mut_ptr()) };
    while let Some(task) = executor.fetch(0) {
        let _ = task.clone().execute();
        if task.task_type == TaskType::KernelSche {
            executor.wake(task.clone());
        }
    }
}
