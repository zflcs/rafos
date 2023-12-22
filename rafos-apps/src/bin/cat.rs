#![no_std]
#![no_main]
#![feature(lang_items)]
#![allow(internal_features)]

#[macros::entry]
pub fn main(argc: usize, argv: &[&str]) -> isize {
    let filename = argv[1];
    let fd = open(filename, OpenFlags::O_RDONLY);
    let mut content = [0u8; 512];
    read(fd as _, &mut content);
    for i in content {
        print!("{}", i as char);
    }
    println!();
    close(fd as _);
    0
}
