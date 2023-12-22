#![no_std]
#![no_main]
#![feature(lang_items)]
#![allow(internal_features)]

#[macros::entry]
pub fn main(argc: usize, argv: &[&str]) -> isize {
    let args = [
        Argument { ch: 'a', rc: 1 },
        Argument { ch: 'b', rc: 2 },
        Argument { ch: 'c', rc: 3 },
    ];
    for arg in args.iter() {
        thread_create(
            thread_print as usize,
            arg as *const _ as _,
        );
    }
    println!("main thread exited.");
    println!("threads with arg test passed!");
    sleep(100);

    0
}


struct Argument {
    pub ch: char,
    pub rc: isize,
}

fn thread_print(arg: *const Argument) -> ! {
    let arg = unsafe { &*arg };
    for _ in 0..1000 {
        print!("{}", arg.ch);
    }
    exit(arg.rc)
}

