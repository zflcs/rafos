

#![feature(allocator_api)]
use core::future::Future;
use core::pin::Pin;
use core::task::Context;
use executor::{TaskType, TaskRef};
use std::time::Instant;
extern crate sys_info;


fn main() {
    // println!("cpu num {}", sys_info::cpu_num().unwrap());
    // println!("cpu speed {}", sys_info::cpu_speed().unwrap());
    // spawn_time_test(100);
    // switch_time_test(100);
    // wake_time_test(5);
    box_drop_test();

}

#[allow(unused)]
fn box_drop_test() {
    
    let lib = unsafe { 
        libloading::Library::new("/home/zfl/workspace/rafos/target/riscv64gc-unknown-linux-gnu/release/librafos_runtime.so") 
    }.unwrap();
    let spawn: libloading::Symbol<extern fn(fut: Box<dyn Future<Output = i32> + 'static + Send + Sync>, priority: u32, task_type: TaskType) -> TaskRef> = 
        unsafe { lib.get(b"spawn") }.unwrap();
    let poll_future: libloading::Symbol<extern fn()> = unsafe { lib.get(b"poll_future") }.unwrap();
    for _ in 0..10 {
        let a = Box::new(test());
        let raw_ptr = Box::into_raw(a);
        println!("raw_ptr {:#X}", raw_ptr as *const usize as usize);
        spawn(unsafe { Box::from_raw(raw_ptr) }, 0, TaskType::Other);
        poll_future();
    }
}

#[allow(unused)]
fn spawn_time_test(num: usize) {
    let lib = unsafe { 
        libloading::Library::new("/home/zfl/u-intr/rafos/target/riscv64gc-unknown-linux-gnu/release/librafos_runtime.so") 
    }.unwrap();
    let spawn: libloading::Symbol<extern fn(fut: Box<dyn Future<Output = i32> + 'static + Send + Sync>, priority: u32, task_type: TaskType) -> TaskRef> = 
        unsafe { lib.get(b"spawn") }.unwrap();
    let mut spawn_times = Vec::new();
    for _ in 0..num {
        let start = Instant::now();
        spawn(Box::new(test()), 2, TaskType::KernelSche);
        spawn_times.push(start.elapsed().as_nanos());
    }
    let poll_future: libloading::Symbol<extern fn()> = unsafe { lib.get(b"poll_future") }.unwrap();
    poll_future();
    println!("spawn future time {:?}", spawn_times);
}

#[allow(unused)]
fn switch_time_test(num: usize) {
    let lib = unsafe { 
        libloading::Library::new("/home/zfl/u-intr/rafos/target/riscv64gc-unknown-linux-gnu/release/librafos_runtime.so") 
    }.unwrap();
    let spawn: libloading::Symbol<extern fn(fut: Box<dyn Future<Output = i32> + 'static + Send + Sync>, priority: u32, task_type: TaskType) -> TaskRef> = 
        unsafe { lib.get(b"spawn") }.unwrap();
    let poll_future: libloading::Symbol<extern fn()> = unsafe { lib.get(b"poll_future") }.unwrap();
    for _ in 0..num {
        spawn(Box::new(Help::new()), 0, TaskType::KernelSche);
        // println!("{:?}", spawn(Box::new(Help::new()), 0, TaskType::KernelSche));
        poll_future();
    }
    
    // println!("poll future end");
}

#[allow(unused)]
fn wake_time_test(num: usize) {
    let lib = unsafe { 
        libloading::Library::new("/home/zfl/u-intr/rafos/target/riscv64gc-unknown-linux-gnu/release/librafos_runtime.so") 
    }.unwrap();
    let spawn: libloading::Symbol<extern fn(fut: Box<dyn Future<Output = i32> + 'static + Send + Sync>, priority: u32, task_type: TaskType) -> TaskRef> = 
        unsafe { lib.get(b"spawn") }.unwrap();
    let poll_future: libloading::Symbol<extern fn()> = unsafe { lib.get(b"poll_future") }.unwrap();
    let wake_task: libloading::Symbol<extern fn(task_ref: TaskRef)> = unsafe { lib.get(b"wake_task") }.unwrap();

    for _ in 0..num {
        let task_ref = spawn(Box::new(Help::new()), 0, TaskType::Other);
        poll_future();
        wake_task(task_ref);
        poll_future();
    }
}

struct Help{
    yield_once: bool,
    time: Instant,
}

impl Help {
    pub fn new() -> Self {
        Self { yield_once: false, time: Instant::now() }
    }
}

impl Future for Help {
    type Output = i32;
    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> core::task::Poll<Self::Output> {
        if !self.yield_once {
            self.yield_once = true;
            self.time = Instant::now();
            core::task::Poll::Pending
        } else {
            println!("switch time {}ns", self.time.elapsed().as_nanos());
            core::task::Poll::Ready(0)
        }
    }
}

async fn test() -> i32 {
    println!("into async test");
    0
}

// async fn test1() -> i32 {
//     println!("into async test1");
//     0
// }

// async fn test2() -> i32 {
//     println!("into async test2");
//     0
// }

// async fn test3() -> i32 {
//     println!("into async test3");
//     0
// }

