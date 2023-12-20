#![no_std]
#![no_main]
#![feature(lang_items)]
#![allow(internal_features)]

use alloc::string::ToString;
use syscall::fork;

static TESTS: &[&str] = &[
    "forktest\0",
    "threads\0",
    "threads_arg\0",
];

#[macros::entry]
pub fn main(argc: usize, argv: &[&str]) -> i32 {
    for (_, &test) in TESTS.iter().enumerate() {
        let pid = fork();
        if pid == 0 {
            if exec(test, &[core::ptr::null::<u8>()]) == -1 {
                println!("Error when executing!");
                return -4;
            }
            unreachable!();
        } else {
            let mut exit_code: i32 = 0;
            let exit_pid = waitpid(pid as usize, &mut exit_code);
            assert_eq!(pid, exit_pid);
        }
    }
    println!("ch5 Usertests passed!");
    0
}