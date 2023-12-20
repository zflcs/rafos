use errno::Errno;
use syscall::{SyscallResult, SyscallTrait};
use crate::{cpu::*, task::{do_fork, do_wait, do_exec, do_thread_create, do_wait_tid}, fs::{open_file, OpenFlags}};
use mmrv::*;

use super::SyscallImpl;

impl SyscallTrait for SyscallImpl {
    fn sys_write(fd:usize, buf_ptr:usize, buf_len:usize) -> SyscallResult {
        let curr = cpu().curr.as_ref().unwrap();
        // Translate user buffer into kernel string.
        let buf = curr.mm().get_buf_mut(VirtAddr::from(buf_ptr), buf_len)?;
        // Get the file with the given file descriptor.
        let file = curr.files().get(fd)?;
        file.write(buf).map_err(|_| Errno::EIO)
    }

    fn sys_read(fd:usize, buf_ptr:usize, buf_len:usize) -> SyscallResult {
        let curr = cpu().curr.as_ref().unwrap();
        // Get the real buffer translated into physical address.
        let buf = curr.mm().get_buf_mut(VirtAddr::from(buf_ptr), buf_len)?;
        // Get the file with the given file descriptor.
        let file = curr.files().get(fd)?;
        file.read(buf).map_err(|_| Errno::EIO)
    }

    fn sys_dup(fd:usize) -> SyscallResult {
        Ok(0)
    }

    fn sys_exit(exit_code:usize) -> SyscallResult {
        unsafe { do_exit(exit_code as _); }
        unreachable!()
    }

    fn sys_fork() -> SyscallResult {
        do_fork()
    }

    fn sys_wait_pid(pid:usize, exit_code_ptr:usize) -> SyscallResult {
        do_wait(pid as _, exit_code_ptr)
    }

    fn sys_exec(path_ptr:usize, _args_ptr:usize) -> SyscallResult {
        let curr = cpu().curr.as_ref().unwrap();
        let path = curr.mm().get_str(VirtAddr::from(path_ptr))?;
        log::trace!("{:?}", path);
        let elf_data = open_file(&path, OpenFlags::RDONLY).unwrap().read_all();
        if do_exec(&elf_data).is_err() {
            Err(Errno::ENOEXEC)
        } else {
            Ok(0)
        }
    }

    fn sys_thread_create(entry:usize, arg:usize) -> SyscallResult {
        do_thread_create(entry, arg)
    }
    
    fn sys_wait_tid(tid:usize) -> SyscallResult {
        do_wait_tid(tid)
    }
}