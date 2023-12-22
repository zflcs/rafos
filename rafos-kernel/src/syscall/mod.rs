
mod file;
mod io;
mod ipc;
mod mm;
mod proc;
mod socket;
mod timer;

use syscall::*;

#[derive(Debug)]
pub struct SyscallArgs(pub SyscallId, pub [usize; 7]);

pub struct SyscallImpl;

pub fn syscall(args: SyscallArgs) -> SyscallResult {
    trace!("[U] SYSCALL {:X?}", args);
    let id = args.0;
    let args = args.1;
    match id {
        SyscallId::File(id) => match id {
            SyscallFile::Read => SyscallImpl::sys_read(args[0], args[1] as _, args[2]),
            SyscallFile::Write => SyscallImpl::sys_write(args[0], args[1] as _, args[2]),
            SyscallFile::Open => SyscallImpl::sys_open(args[0] as _, args[1], args[2]),
            SyscallFile::Close => SyscallImpl::sys_close(args[0]),
            SyscallFile::Stat => todo!(),
            SyscallFile::FsStat => todo!(),
            SyscallFile::LsStat => todo!(),
            SyscallFile::LsSeek => todo!(),
            SyscallFile::Pread64 => todo!(),
            SyscallFile::Pwrite64 => todo!(),
            SyscallFile::Readv => todo!(),
            SyscallFile::Writev => todo!(),
            SyscallFile::Access => todo!(),
            SyscallFile::Dup => todo!(),
            SyscallFile::Dup2 => todo!(),
            SyscallFile::SendFile => todo!(),
            SyscallFile::Getdents => SyscallImpl::sys_getdents(args[0], args[1] as _, args[2]),
        },
        SyscallId::Io(id) => match id {
            SyscallIO::Ioctl => todo!(),
        },
        SyscallId::Ipc(id) => match id {
            SyscallIPC::SigAction => todo!(),
            SyscallIPC::SigProcMask => todo!(),
            SyscallIPC::SigReturn => todo!(),
            SyscallIPC::Pipe => todo!(),
            SyscallIPC::Kill => todo!(),
        },
        SyscallId::Mm(id) =>  match id {
            SyscallMM::Mmap => todo!(),
            SyscallMM::Mprotect => todo!(),
            SyscallMM::Munmap => todo!(),
            SyscallMM::Brk => todo!(),
            SyscallMM::Mremap => todo!(),
            SyscallMM::Msync => todo!(),
            SyscallMM::MinCore => todo!(),
            SyscallMM::Madvise => todo!(),
            SyscallMM::Shmget => todo!(),
            SyscallMM::Shmat => todo!(),
            SyscallMM::Shmctl => todo!(),
        },
        SyscallId::Proc(id) => match id {
            SyscallProc::SchedYield => todo!(),
            SyscallProc::Pause => todo!(),
            SyscallProc::GetPid => todo!(),
            SyscallProc::Clone => SyscallImpl::sys_clone(args[0], args[1], args[2], args[3] as _, args[4], args[5], args[6]),
            SyscallProc::Fork => SyscallImpl::sys_fork(),
            SyscallProc::Vfork => todo!(),
            SyscallProc::Execve => SyscallImpl::sys_execve(args[0] as _, args[1] as _, args[2] as _),
            SyscallProc::Exit => SyscallImpl::sys_exit(args[0] as _),
            SyscallProc::Wait4 => SyscallImpl::sys_wait4(args[0] as _, args[1] as _, args[2], args[3]),
        },
        SyscallId::Socket(id) => match id {
            SyscallSocket::Socket => todo!(),
            SyscallSocket::Connect => todo!(),
            SyscallSocket::Accept => todo!(),
        },
        SyscallId::Timer(id) => match id {
            SyscallTimer::NanoSleep => SyscallImpl::sys_nano_sleep(args[0] as _, args[1] as _),
            SyscallTimer::Getitimer => todo!(),
            SyscallTimer::Alarm => todo!(),
            SyscallTimer::Setitimer => todo!(),
        },
    }
}