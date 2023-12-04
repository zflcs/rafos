pub mod const_reloc;
pub mod manager;
pub mod structs;
pub mod api;
use executor::*;
pub use manager::LKM_MANAGER;

use core::future::Future;
use alloc::boxed::Box;

pub fn init() {
    let _ = LKM_MANAGER.lock();
}

pub fn spawn(fut: Box<dyn Future<Output = i32> + 'static + Send + Sync>, priority: u32, task_type: TaskType) -> TaskRef {
    let spawn_ptr = LKM_MANAGER.lock().resolve_symbol("spawn").unwrap();
    unsafe {
        let spawn_fn: fn(Box<dyn Future<Output = i32> + 'static + Send + Sync>, u32, TaskType) -> TaskRef = core::mem::transmute(spawn_ptr);
        spawn_fn(fut, priority, task_type)
    }
}

pub fn poll_future() {
    let poll_future_ptr = LKM_MANAGER.lock().resolve_symbol("poll_future").unwrap();
    unsafe {
        let poll_future_fn: fn() = core::mem::transmute(poll_future_ptr);
        poll_future_fn();
    }
}

pub fn wake_task(task_ref: TaskRef) {
    let wake_task_ptr = LKM_MANAGER.lock().resolve_symbol("wake_task").unwrap();
    unsafe {
        let wake_task_fn: fn(TaskRef) = core::mem::transmute(wake_task_ptr);
        wake_task_fn(task_ref);
    }
}

pub fn put_test() {
    let put_test_ptr = LKM_MANAGER.lock().resolve_symbol("put_test").unwrap();
    unsafe {
        let put_test_fn: fn() = core::mem::transmute(put_test_ptr);
        put_test_fn();
    }
}
