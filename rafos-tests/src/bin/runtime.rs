


use core::future::Future;
use executor::TaskType;

fn main() {
    let lib = unsafe { 
        libloading::Library::new("/home/zfl/u-intr/rafos/target/riscv64gc-unknown-linux-gnu/release/librafos_runtime.so") 
    }.unwrap();
    let spawn: libloading::Symbol<extern fn(fut: Box<dyn Future<Output = i32> + 'static + Send + Sync>, priority: u32, task_type: TaskType)> = 
        unsafe { lib.get(b"spawn") }.unwrap();
    let poll_future: libloading::Symbol<extern fn()> = unsafe { lib.get(b"poll_future") }.unwrap();
    spawn(Box::new(test2()), 2, TaskType::Other);
    spawn(Box::new(test3()), 3, TaskType::Other);
    spawn(Box::new(test()), 0, TaskType::Other);
    spawn(Box::new(test1()), 1, TaskType::Other);
    poll_future();
}

async fn test() -> i32 {
    println!("into async test");
    0
}

async fn test1() -> i32 {
    println!("into async test1");
    0
}

async fn test2() -> i32 {
    println!("into async test2");
    0
}

async fn test3() -> i32 {
    println!("into async test3");
    0
}

