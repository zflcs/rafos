use core::fmt::Write;

use super::Console;


/// The requirement of `core::fmt::Write` trait
impl Write for Console {
    #[inline]
    fn write_str(&mut self, s: &str) -> Result<(), core::fmt::Error> {
        let buf = s.as_bytes();
        syscall::sys_write(syscall::STDOUT, buf.as_ptr() as _, buf.len());
        Ok(())
    }
}