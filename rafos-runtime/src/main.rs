
#![no_std]
#![no_main]

extern crate asyncc;
use asyncc::*;

#[export_name = "_start"]
pub fn entry() {
    let asyncc =  asyncc::get_asyncc();
    let cur_task = asyncc.get_curr();
    match asyncc.cause() {
        Cause::Finish => do_finish(),
        Cause::Await => do_await(asyncc, cur_task),
        Cause::Intr(_) => do_intr(),
        Cause::Exception(_) => do_exception(),
    }
}

/// the coroutine is finished, so we don't need to save it, and the 
pub fn do_finish() {

}

/// The coroutine is not finished but yield actively, 
/// so we need to deal with it according to it's type.
/// 1. KernelSche: This kind task must be waked immediately
/// 2. Syscall: This task must will not be waked immediately, but we need to send message to the buffer and need to trap into kernel
/// 3. AsyncSyscall: It's similar to `Syscall`, but it will not result in trapping into kernel
/// 4. Other: It's the normal task, but it need to wait for event to be waked, so it will do nothing
pub fn do_await(asyncc: &Asyncc, task_ref: TaskRef) {
    let task_type = (unsafe { &*task_ref.as_ptr() }).task_type;
    match task_type {
        TaskType::KernelSche => { wake_task(task_ref) },
        TaskType::Syscall => {
            let queue = asyncc.get_msgqueue();
            while queue.enqueue(task_ref).is_err() {}
        },
        TaskType::AsyncSyscall => {
            let queue = asyncc.get_msgqueue();
            while queue.enqueue(task_ref).is_err() {}
        },
        TaskType::Other => {},
    }
}

pub fn do_intr() {
    todo!()
}

pub fn do_exception() {
    todo!()
}


use core::alloc::GlobalAlloc;
struct Global;

#[global_allocator]
static GLOBAL: Global = Global;

unsafe impl GlobalAlloc for Global {
    unsafe fn alloc(&self, _layout: core::alloc::Layout) -> *mut u8 {
        unreachable!()
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: core::alloc::Layout) {
        unreachable!()
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unreachable!()
}

use core::future::Future;

pub fn test(_a: *mut (dyn Future<Output = i32> + 'static + Send + Sync)) {
    
}