#![no_std]
#![no_main]
#![feature(lang_items)]
#![allow(internal_features)]

#[macros::entry]
pub fn main(argc: usize, argv: &[&str]) -> isize {
    if argv.len() == 1 {
        parse_args("./\0")
    } else {
        let path = argv[1];
        parse_args(path)
    }
    0
}

const BUF_SIZE:usize = 512;
fn parse_args(path: &str) {
    let fd = open(path, OpenFlags::O_DIRECTORY | OpenFlags::O_DSYNC);
    assert!(fd >= 0, "open failed");
    let mut buf = [0u8; BUF_SIZE];
    getdents(fd as usize, &mut buf);
}

