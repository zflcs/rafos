
mod r#impl;

use syscall::*;

#[derive(Debug)]
pub struct SyscallArgs(pub SyscallId, pub [usize; 6]);

pub struct SyscallImpl;

pub fn syscall(args: SyscallArgs) -> SyscallResult {
    trace!("[U] SYSCALL {:X?}", args);
    let id = args.0;
    let args = args.1;
    match id {
        SyscallId::Dup => todo!(),
        SyscallId::Open => todo!(),
        SyscallId::Close => todo!(),
        SyscallId::Pipe => todo!(),
        SyscallId::Read => SyscallImpl::sys_read(args[0], args[1], args[2]),
        SyscallId::Write => SyscallImpl::sys_write(args[0], args[1], args[2]),
        SyscallId::Exit => SyscallImpl::sys_exit(args[0]),
        SyscallId::Yield => todo!(),
        SyscallId::GetTime => todo!(),
        SyscallId::GetPid => todo!(),
        SyscallId::Fork => SyscallImpl::sys_fork(),
        SyscallId::Exec => SyscallImpl::sys_exec(args[0], args[1]),
        SyscallId::WaitPid => SyscallImpl::sys_wait_pid(args[0], args[1]),
        SyscallId::Spawn => todo!(),
        SyscallId::MailRead => todo!(),
        SyscallId::MailWrite => todo!(),
        SyscallId::FlushTrace => todo!(),
        SyscallId::InitUserTrap => todo!(),
        SyscallId::SendMsg => todo!(),
        SyscallId::SetTimer => todo!(),
        SyscallId::ClaimExtInt => todo!(),
        SyscallId::SetExtIntEnable => todo!(),
        SyscallId::ThreadCreate => SyscallImpl::sys_thread_create(args[0], args[1]),
        SyscallId::GetTid => todo!(),
        SyscallId::WaitTid => SyscallImpl::sys_wait_tid(args[0]),
        SyscallId::Hang => todo!(),
        SyscallId::MutexCreate => todo!(),
        SyscallId::MutexLock => todo!(),
        SyscallId::MutexUnlock => todo!(),
        SyscallId::CondvarCreate => todo!(),
        SyscallId::CondvarSignal => todo!(),
        SyscallId::CondvarWait => todo!(),
        SyscallId::Listen => todo!(),
        SyscallId::Accept => todo!(),
        SyscallId::UintrTest => todo!(),
    }
}