
use super::SyscallImpl;
use crate::{cpu::cpu, task::{do_wait_tid, do_wait, do_exec, do_fork, do_thread_create, do_exit, do_clone}, fs::open, read_user};
use alloc::{vec::Vec, string::String};
use errno::Errno;
use mmrv::VirtAddr;
use syscall::*;
use vfs::{Path, OpenFlags};

impl SyscallProcTrait for SyscallImpl {

    fn sys_exit(exit_code: isize) -> SyscallResult {
        unsafe { do_exit(exit_code); }
        unreachable!()
    }

    fn sys_clone(entry:usize, stack:usize, flags:usize, arg: *const usize, ptid:usize, tls:usize, ctid:usize) -> SyscallResult {
        let clone_flags = CloneFlags::from_bits(flags);
        if clone_flags.is_none() {
            return Err(Errno::EINVAL);
        }
        do_clone(
            entry,
            clone_flags.unwrap(), 
            stack, 
            arg,
            VirtAddr::from(ptid), 
            tls, 
            VirtAddr::from(ctid)
        )
    }

    fn sys_fork() -> SyscallResult {
        do_fork()
    }

    fn sys_waittid(tid:usize) -> SyscallResult {
        do_wait_tid(tid)
    }

    fn sys_waitpid(pid:usize, exit_code_ptr: *mut isize) -> SyscallResult {
        do_wait(pid as _, exit_code_ptr as _)
    }

    fn sys_execve(filename: *const u8, argv: *const usize, _envp: *const usize) -> SyscallResult {
        let curr = cpu().curr.as_ref().unwrap();
        // get relative path under current working directory
        let rela_path = curr.mm().get_str(VirtAddr::from(filename as usize))?;
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
        let mut argv = argv as usize;
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
        log::trace!("{:?} {:#X?}", argc, argv);
        log::trace!("{:?}", path);
        do_exec(String::from(path.as_str()), elf_data.as_slice(), args)
    }

    fn sys_thread_create(entry:usize, arg: *const usize) -> SyscallResult {
        do_thread_create(entry, arg as _)
    }
    
}


