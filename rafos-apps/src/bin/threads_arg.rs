#![no_std]
#![no_main]
#![feature(lang_items)]
#![allow(internal_features)]

#[macros::entry]
pub fn main(argc: usize, argv: &[&str]) -> isize {
    let mut v = Vec::new();
    let args = [
        Argument { ch: 'a', rc: 1 },
        Argument { ch: 'b', rc: 2 },
        Argument { ch: 'c', rc: 3 },
    ];
    for arg in args.iter() {
        v.push(thread_create(
            thread_print as usize,
            arg as *const _ as _,
        ));
    }
    for tid in v.iter() {
        let exit_code = waittid(*tid as usize);
        println!("thread#{} exited with code {}", tid, exit_code);
    }
    println!("main thread exited.");
    println!("threads with arg test passed!");
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

