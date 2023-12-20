use alloc::{vec::Vec, string::String};
use errno::Errno;
use syscall::{SyscallResult, SyscallTrait};
use vfs::{OpenFlags, Path};
use crate::{cpu::*, task::{do_fork, do_wait, do_exec, do_thread_create, do_wait_tid}, fs::open, read_user};
use mmrv::*;

use super::SyscallImpl;

impl SyscallTrait for SyscallImpl {
    fn sys_write(fd:usize, buf_ptr:usize, buf_len:usize) -> SyscallResult {
        let curr = cpu().curr.as_ref().unwrap();
        // Translate user buffer into kernel string.
        let buf = curr.mm().get_buf_mut(VirtAddr::from(buf_ptr), buf_len)?;
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

    fn sys_read(fd:usize, buf_ptr:usize, buf_len:usize) -> SyscallResult {
        let curr = cpu().curr.as_ref().unwrap();
        // Get the real buffer translated into physical address.
        let buf = curr.mm().get_buf_mut(VirtAddr::from(buf_ptr), buf_len)?;
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

    fn sys_exec(path_ptr:usize, args_ptr:usize) -> SyscallResult {
        let curr = cpu().curr.as_ref().unwrap();
        // get relative path under current working directory
        let rela_path = curr.mm().get_str(VirtAddr::from(path_ptr))?;
        // get absolute path of the file to execute
        let fs_info = curr.fs_info.lock();
        let mut path = Path::from(fs_info.cwd.clone() + "/" + rela_path.as_str());
        drop(fs_info);
        // read file from disk
        let file = open(path.clone(), OpenFlags::O_RDONLY)?;
        if !file.is_reg() {
            return Err(Errno::EACCES);
        }
        let elf_data = unsafe { file.read_all() };
        // get argument list
        let mut args = Vec::new();
        let mut argv = args_ptr;
        let mut argc: usize = 0;
        let mut curr_mm = curr.mm();
        loop {
            read_user!(curr_mm, VirtAddr::from(argv), argc, usize)?;
            if argc == 0 {
                break;
            }
            args.push(curr_mm.get_str(VirtAddr::from(argc))?);
            argv += core::mem::size_of::<usize>();
        }
        drop(curr_mm);
        path.pop().unwrap(); // unwrap a regular filename freely
        log::trace!("{:?} {:#X?}", path_ptr, args_ptr);
        log::trace!("{:?}", path);
        do_exec(String::from(path.as_str()), elf_data.as_slice(), args)
    }

    fn sys_thread_create(entry:usize, arg:usize) -> SyscallResult {
        do_thread_create(entry, arg)
    }
    
    fn sys_wait_tid(tid:usize) -> SyscallResult {
        do_wait_tid(tid)
    }
}