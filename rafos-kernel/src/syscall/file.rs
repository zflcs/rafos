use mmrv::*;
use crate::cpu::cpu;
use syscall::*;

use super::SyscallImpl;

impl SyscallFileTrait for SyscallImpl {
    fn sys_write(fd:usize, buf: *const u8, count:usize) -> SyscallResult {
        let curr = cpu().curr.as_ref().unwrap();
        // Translate user buffer into kernel string.
        let buf = curr.mm().get_buf_mut(VirtAddr::from(buf as usize), count)?;
        // Get the file with the given file descriptor.
        let file = curr.files().get(fd)?;
        let mut write_len = 0;
        for bytes in buf.inner {
            if let Some(count) = file.write(bytes) {
                write_len += count;
            } else {
                break;
            }
        }
        Ok(write_len)
    }

    fn sys_read(fd:usize, buf: *mut u8, count:usize) -> SyscallResult {
        let curr = cpu().curr.as_ref().unwrap();
        // Get the real buffer translated into physical address.
        let buf = curr.mm().get_buf_mut(VirtAddr::from(buf as usize), count)?;
        // Get the file with the given file descriptor.
        let file = curr.files().get(fd)?;
        let mut read_len = 0;
        for bytes in buf.inner {
            if let Some(count) = file.read(bytes) {
                read_len += count;
            } else {
                break;
            }
        }
        Ok(read_len)
    }

    fn sys_close(fd:usize) -> SyscallResult {
        cpu().curr.as_ref().unwrap().files().remove(fd)?;
        Ok(0)
    }
}

