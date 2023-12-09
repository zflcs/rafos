#![no_std]
#![no_main]
#![feature(lang_items, noop_waker)]
#![allow(internal_features, non_snake_case)]

extern crate alloc;
use alloc::boxed::Box;
use asyncc::*;

/// This function need to be defined in kernel or user process. 
#[link_section = ".text.entry"]
#[no_mangle]
pub fn _start(task_ref: Option<TaskRef>) -> Option<TaskRef> {
    if let Some(task_ref) = task_ref {
        unsafe {
            asyncc::Asyncc::set_curr(Some(task_ref));
            let waker = asyncc::from_task(task_ref);
            let mut cx = Context::from_waker(&waker);
            let task = Task::from_ref(task_ref);
            task.state.store(TaskState::Running as _, Ordering::Relaxed);
            let fut = &mut *task.fut.as_ptr();
            let mut future = Pin::new_unchecked(fut.as_mut());
            match future.as_mut().poll(&mut cx) {
                Poll::Ready(_) => { 
                    Asyncc::set_cause(crate::Cause::Finish);
                    None },
                Poll::Pending => { 
                    Asyncc::set_cause(crate::Cause::Await);
                    task.state.store(TaskState::Pending as _, Ordering::Relaxed);
                    Some(task.as_ref()) 
                },
            }
        }
    } else {
        let executor = asyncc::Asyncc::get_executor();
        if executor.state.load(Ordering::Relaxed) == ExecutorState::Ready as _ {
            Asyncc::spawn(Box::new(main(0)), 0, asyncc::TaskType::Other);
            executor.state.store(ExecutorState::Running as _, Ordering::Relaxed);
        }
        None
    }
}

pub async fn main(args: usize) -> i32 {
    // let a = alloc::boxed::Box::pin(test());
    // let waker = Waker::noop();
    // let mut cx = Context::from_waker(&waker);
    // a.await
    // test().await
    let task = alloc::boxed::Box::new(Help::new());
    task.await;
    args as _
}

struct Help{
    yield_once: bool,
}

impl Help {
    pub fn new() -> Self {
        Self { yield_once: false }
    }
}
use core::future::Future;
use core::pin::Pin;
use core::ptr::NonNull;
use core::sync::atomic::Ordering;
use core::task::{Context, Poll};
impl Future for Help {
    type Output = i32;
    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> core::task::Poll<Self::Output> {
        if !self.yield_once {
            self.yield_once = true;
            core::task::Poll::Pending
        } else {
            core::task::Poll::Ready(0)
        }
    }
}

///
#[lang = "eh_personality"]
#[no_mangle]
pub fn rust_eh_personality() {}

#[no_mangle]
pub fn _Unwind_Resume() {}


use core::alloc::GlobalAlloc;

use buddy_system_allocator::LockedHeap;
struct Global;

#[global_allocator]
static GLOBAL: Global = Global;
const USER_HEAP_PTR: usize = 0xFFFFE000;

unsafe impl GlobalAlloc for Global {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        let allocator = &*(USER_HEAP_PTR as *const usize as *const LockedHeap<32>);
        allocator.lock()
            .alloc(layout)
            .ok()
            .map_or(0 as *mut u8, |allocation| allocation.as_ptr())
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: core::alloc::Layout) {
        let allocator = &*(USER_HEAP_PTR as *const usize as *const LockedHeap<32>);
        allocator.lock().dealloc(NonNull::new_unchecked(ptr), layout)
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unreachable!()
}
