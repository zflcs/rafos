

use time::*;
#[allow(unused)]
pub fn box_drop_test(num: usize) {
    use alloc::boxed::Box;
    for _ in 0..num {
        let a = Box::new(test());
        let raw_ptr = Box::into_raw(a);
        log::debug!("raw_ptr {:#X}", raw_ptr as *const usize as usize);
        crate::lkm::spawn(unsafe { Box::from_raw(raw_ptr) }, 0, asyncc::TaskType::Other);
        crate::lkm::poll_future();
    }
}

#[allow(unused)]
pub fn spawn_time_test(num: usize) {
    log::debug!("spawn_time_test");
    use alloc::{boxed::Box, vec::Vec};
    let mut spawn_times = Vec::new();
    for _ in 0..num {
        let start = Instant::now();
        crate::lkm::spawn(Box::new(test()), 2, asyncc::TaskType::KernelSche);
        spawn_times.push(start.elapsed().as_nanos());
    }
    crate::lkm::poll_future();
    log::debug!("spawn future time {:?}", spawn_times);
}

#[allow(unused)]
pub fn switch_time_test(num: usize) {
    use alloc::boxed::Box;
    for _ in 0..num {
        crate::lkm::spawn(Box::new(Help::new()), 0, asyncc::TaskType::KernelSche);
        // println!("{:?}", spawn(Box::new(Help::new()), 0, TaskType::KernelSche));
        crate::lkm::poll_future();
    }
    
    // println!("poll future end");
}

#[allow(unused)]
pub fn wake_time_test(num: usize) {
    use alloc::boxed::Box;
    for _ in 0..num {
        let task_ref = crate::lkm::spawn(Box::new(Help::new()), 0, asyncc::TaskType::Other);
        crate::lkm::poll_future();
        crate::lkm::wake_task(task_ref);
        crate::lkm::poll_future();
    }
}
use core::future::Future;

struct Help{
    yield_once: bool,
    time: Instant,
}

impl Help {
    pub fn new() -> Self {
        Self { yield_once: false, time: Instant::now() }
    }
}
use core::task::Context;
use core::pin::Pin;
impl Future for Help {
    type Output = i32;
    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> core::task::Poll<Self::Output> {
        if !self.yield_once {
            self.yield_once = true;
            self.time = Instant::now();
            core::task::Poll::Pending
        } else {
            log::debug!("switch time {}ns", self.time.elapsed().as_nanos());
            core::task::Poll::Ready(0)
        }
    }
}

#[no_mangle]
pub async fn test() -> i32 {
    println!("into async test");
    0
}    

