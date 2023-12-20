#![no_std]
#![no_main]
#![feature(lang_items)]
#![allow(internal_features)]


#[macros::entry]
pub fn main() -> i32 {
    let i  = 43;
    println!("{}", i * 2);
    0
}