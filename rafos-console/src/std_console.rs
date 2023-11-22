use core::fmt::Write;

use super::Console;
extern crate std;
use std::println;
/// The requirement of `core::fmt::Write` trait
impl Write for Console {
    #[inline]
    fn write_str(&mut self, s: &str) -> Result<(), core::fmt::Error> {
        println!("{}", s);
        Ok(())
    }
}