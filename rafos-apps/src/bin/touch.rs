#![no_std]
#![no_main]
#![feature(lang_items)]
#![allow(internal_features)]

#[macros::entry]
pub fn main(argc: usize, argv: &[&str]) -> isize {
    let filename = argv[1];
    let fd = open(filename, OpenFlags::O_CREAT | OpenFlags::O_RDWR);
    let content = "hello file";
    write(fd as _, content.as_bytes());
    close(fd as _);
    0
}
