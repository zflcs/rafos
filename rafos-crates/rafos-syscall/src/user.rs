pub use vfs::OpenFlags;
use crate::*;

pub fn dup(fd: usize) -> isize {
    sys_dup(fd)
}


pub fn open(filename: &str, flags: OpenFlags) -> isize {
    sys_open(filename.as_ptr(), flags.bits() as _, 0)
}

pub fn close(fd: usize) -> isize {
    sys_close(fd)
}

// pub fn pipe(pipe: &mut [usize]) -> isize {
//     sys_pipe(pipe.as_mut_ptr())
// }

pub fn read(fd: usize, buf: &mut [u8]) -> isize {
    sys_read(fd, buf.as_mut_ptr(), buf.len())
}

pub fn write(fd: usize, buf: &[u8]) -> isize {
    sys_write(fd, buf.as_ptr(), buf.len())
}

pub fn exit(exit_code: isize) -> ! {
    sys_exit(exit_code);
    panic!("sys_exit never returns!");
}

pub fn sched_yield() -> isize {
    sys_sched_yield()
}

pub fn getpid() -> isize {
    sys_get_pid()
}

pub fn fork() -> isize {
    sys_fork()
}

pub fn exec(filename: &str, args: &[*const u8], envs: &[*const u8]) -> isize {
    sys_execve(filename.as_ptr(), args.as_ptr() as *const usize, envs.as_ptr() as *const usize)
}


pub fn wait(exit_code_ptr: *mut isize) -> isize {
    sys_waitpid(usize::MAX, exit_code_ptr)
}

pub fn waitpid(pid: usize, exit_code_ptr: &mut isize) -> isize {
    sys_waitpid(pid, exit_code_ptr)
    
}

pub fn waittid(tid: usize) -> isize {
    sys_waittid(tid)
}

pub fn sleep(period_ms: usize) {
    // sys_nano_sleep(rqtp, rmtp)

}




pub fn thread_create(entry: usize, arg: *const usize) -> isize {
    sys_thread_create(entry, arg)
}

