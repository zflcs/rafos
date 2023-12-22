#![no_std]
#![no_main]
#![feature(lang_items)]
#![allow(internal_features)]


#[macros::entry]
pub fn main(argc: usize, argv: &[&str]) -> isize {
    thread_create(thread_a as usize, core::ptr::null());
    thread_create(thread_b as usize, core::ptr::null());
    thread_create(thread_c as usize, core::ptr::null());
    println!("main thread exited.");
    println!("threads test passed!");
    sleep(100);
    0
}


pub fn thread_a() -> ! {
    let mut t = 2i32;
    for _ in 0..1000 {
        print!("a");
        for __ in 0..5000 {
            t = t * t % 10007;
        }
    }
    println!("{}", t);
    exit(1)
}

pub fn thread_b() -> ! {
    let mut t = 2i32;
    for _ in 0..1000 {
        print!("b");
        for __ in 0..5000 {
            t = t * t % 10007;
        }
    }
    println!("{}", t);
    exit(2)
}

pub fn thread_c() -> ! {
    let mut t = 2i32;
    for _ in 0..1000 {
        print!("c");
        for __ in 0..5000 {
            t = t * t % 10007;
        }
    }
    println!("{}", t);
    exit(3)
}

