//! 
//! 
#![no_std]
#![no_main]
#![feature(panic_info_message, alloc_error_handler, lang_items, naked_functions, asm_const)]
#![allow(internal_features, non_snake_case)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate console;
extern crate alloc;
extern crate rv_plic;




mod lang_item;
mod heap;
mod frame_allocator;
mod mm;
mod lkm;
mod error;
mod fs;
mod device;
mod timer;

pub use frame_allocator::*;
pub use error::*;

use core::sync::atomic::{Ordering, AtomicUsize};
use config::CPU_NUM;





core::arch::global_asm!(include_str!("ramfs.asm"));

/// Boot kernel size allocated in `_start` for single CPU.
pub const BOOT_STACK_SIZE: usize = 0x4_0000;

/// Total boot kernel size.
pub const TOTAL_BOOT_STACK_SIZE: usize = BOOT_STACK_SIZE * CPU_NUM;

/// Initialize kernel stack in .bss section.
#[link_section = ".bss.stack"]
static mut STACK: [u8; TOTAL_BOOT_STACK_SIZE] = [0u8; TOTAL_BOOT_STACK_SIZE];

/// Entry for the first kernel.
#[naked]
#[no_mangle]
#[link_section = ".text.entry"]
pub unsafe extern "C" fn _start(hartid: usize) -> ! {
    core::arch::asm!(
        // Use tp to save hartid
        "mv tp, a0",
        // Set stack pointer to the kernel stack.
        "
        la a1, {stack}
        li t0, {total_stack_size}
        li t1, {stack_size}
        mul sp, a0, t1
        sub sp, t0, sp
        add sp, a1, sp
        ",        // Jump to the main function.
        "j  {main}",
        total_stack_size = const TOTAL_BOOT_STACK_SIZE,
        stack_size       = const BOOT_STACK_SIZE,
        stack            =   sym STACK,
        main             =   sym rust_main_init,
        options(noreturn),
    )
}

/// Entry for other kernels.
#[naked]
#[no_mangle]
pub unsafe extern "C" fn __entry_others(hartid: usize) -> ! {
    core::arch::asm!(
        // Use tp to save hartid
        "mv tp, a0",
        // Set stack pointer to the kernel stack.
        "
        la a1, {stack}
        li t0, {total_stack_size}
        li t1, {stack_size}
        mul sp, a0, t1
        sub sp, t0, sp
        add sp, a1, sp
        ",
        // Jump to the main function.
        "j  {main}",
        total_stack_size = const TOTAL_BOOT_STACK_SIZE,
        stack_size       = const BOOT_STACK_SIZE,
        stack            =   sym STACK,
        main             =   sym rust_main_init_other,
        options(noreturn),
    )
}

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) });
}

static BOOT_HART: AtomicUsize = AtomicUsize::new(0);

#[no_mangle]
pub fn rust_main_init(hart_id: usize) -> ! {
    clear_bss();
    console::init(option_env!("LOG"));
    heap::init_heap();
    frame_allocator::init_frame_allocator();
    mm::init();
    BOOT_HART.fetch_add(1, Ordering::Relaxed);
    fs::list_apps();
    lkm::init();
    
    // net::init();
    // device::init();
    // plic::init();
    // plic::init_hart(hart_id);


    // if CPU_NUM > 1 {
    //     for i in 0..CPU_NUM {
    //         let boot_hart_cnt = BOOT_HART.load(Ordering::Relaxed);
    //         if i != hart_id {
    //             // Starts other harts.
    //             let ret = sbi_rt::hart_start(i, __entry_others as _, 0);
    //             assert!(ret.is_ok(), "Failed to shart hart {}", i);
    //             while BOOT_HART.load(Ordering::Relaxed) == boot_hart_cnt {}
    //         }
    //     }
    // }
    rust_main(hart_id)
}

#[no_mangle]
pub fn rust_main_init_other(hart_id: usize) -> ! {
    BOOT_HART.fetch_add(1, Ordering::Relaxed);
    mm::init();
    rust_main(hart_id)
}

#[no_mangle]
pub fn rust_main(_hart_id: usize) -> ! {
    // lkm::spawn(alloc::boxed::Box::new(async {
    //     log::debug!("async task");
    //     0
    // }), 0, executor::TaskType::KernelSche);
    // let a = alloc::vec![1, 2, 3, 4];
    // log::debug!("{:?}", a);
    // // lkm::put_test();
    // lkm::poll_future();
    // box_drop_test(10000);
    // spawn_time_test(10000);
    // switch_time_test(10000);
    // wake_time_test(10000);
    panic!("Unreachable in rust_main!");
}

#[no_mangle]
fn put_str(ptr: *const u8, len: usize) {
    let bytes = unsafe { core::slice::from_raw_parts(ptr, len) };
    for c in bytes {
        #[allow(deprecated)]
        sbi_rt::legacy::console_putchar(*c as _);
    }
}

mod test {

    use time::*;
    #[allow(unused)]
    fn box_drop_test(num: usize) {
        use alloc::boxed::Box;
        for _ in 0..num {
            let a = Box::new(test());
            let raw_ptr = Box::into_raw(a);
            log::debug!("raw_ptr {:#X}", raw_ptr as *const usize as usize);
            crate::lkm::spawn(unsafe { Box::from_raw(raw_ptr) }, 0, executor::TaskType::Other);
            crate::lkm::poll_future();
        }
    }
    
    #[allow(unused)]
    fn spawn_time_test(num: usize) {
        use alloc::{boxed::Box, vec::Vec};
        let mut spawn_times = Vec::new();
        for _ in 0..num {
            let start = Instant::now();
            crate::lkm::spawn(Box::new(test()), 2, executor::TaskType::KernelSche);
            spawn_times.push(start.elapsed().as_nanos());
        }
        crate::lkm::poll_future();
        log::debug!("spawn future time {:?}", spawn_times);
    }
    
    #[allow(unused)]
    fn switch_time_test(num: usize) {
        use alloc::boxed::Box;
        for _ in 0..num {
            crate::lkm::spawn(Box::new(Help::new()), 0, executor::TaskType::KernelSche);
            // println!("{:?}", spawn(Box::new(Help::new()), 0, TaskType::KernelSche));
            crate::lkm::poll_future();
        }
        
        // println!("poll future end");
    }
    
    #[allow(unused)]
    fn wake_time_test(num: usize) {
        use alloc::boxed::Box;
        for _ in 0..num {
            let task_ref = crate::lkm::spawn(Box::new(Help::new()), 0, executor::TaskType::Other);
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
    
    async fn test() -> i32 {
        println!("into async test");
        0
    }    
}
