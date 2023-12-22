use errno::Errno;
use mmrv::*;
use vfs::{Path, OpenFlags, StatMode};
use crate::{cpu::cpu, fs::open};
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

    fn sys_getdents(fd:usize, buf: *mut u8, count:usize) -> SyscallResult {
        log::trace!("[getdents] fd: {}, buf size: {}", fd, count);
        let curr = cpu().curr.as_ref().unwrap();
        let file = curr.files().get(fd).map_err(|_| Errno::EBADE)?;
        let buf = curr.mm().get_buf_mut(VirtAddr::from(buf as usize), count)?;
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

    fn sys_open(filename: *const u8, flags:usize, mode:usize) -> SyscallResult {
        let flags = OpenFlags::from_bits(flags as u32);
        let mode = StatMode::from_bits(mode as u32);
        if flags.is_none() {
            return Err(Errno::EINVAL);
        }
        let flags = flags.unwrap();
        if flags.contains(OpenFlags::O_CREAT) && mode.is_none()
            || flags.contains(OpenFlags::O_WRONLY | OpenFlags::O_RDWR)
        {
            return Err(Errno::EINVAL);
        }
        let curr = cpu().curr.as_ref().unwrap();
        let rela_path = curr.mm().get_str(VirtAddr::from(filename as usize))?;
        // get absolute path of the file to execute
        let fs_info = curr.fs_info.lock();
        let path = Path::from(fs_info.cwd.clone() + "/" + rela_path.as_str());
        drop(fs_info);
        // read file from disk
        log::trace!("{:?}", path);
        let file = open(path.clone(), flags)?;
        Ok(curr.files().push(file)?)
    }
}

