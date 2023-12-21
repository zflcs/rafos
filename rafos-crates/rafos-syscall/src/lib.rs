//! generate the user syscall interface and the kernel syscall trait.

#![no_std]
#![feature(linked_list_remove)]
#![allow(unused_variables)]

extern crate alloc;
extern crate macros;
mod io;
mod file;
mod ipc;
mod proc;
mod timer;
mod mm;
mod socket;
mod sig_defs;

pub use sig_defs::*;
pub use io::*;
pub use file::*;
pub use ipc::*;
pub use proc::*;
pub use timer::*;
pub use mm::*;
pub use socket::*;

use macros::syscall;
use errno::Errno;
pub type SyscallResult = Result<usize, Errno>;


#[cfg(feature = "user")]
mod user;
#[cfg(feature = "user")]
pub use user::*;

syscall!(a7);
syscall!(a7, a0);
syscall!(a7, a0, a1);
syscall!(a7, a0, a1, a2);
syscall!(a7, a0, a1, a2, a3);
syscall!(a7, a0, a1, a2, a3, a4);
syscall!(a7, a0, a1, a2, a3, a4, a5);
syscall!(a7, a0, a1, a2, a3, a4, a5, a6);

#[repr(C)]
#[derive(Debug)]
pub enum SyscallId {
    File(SyscallFile),
    Io(SyscallIO),
    Ipc(SyscallIPC),
    Mm(SyscallMM),
    Proc(SyscallProc),
    Socket(SyscallSocket),
    Timer(SyscallTimer),
}


impl TryFrom<usize> for SyscallId {
    type Error = usize;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        if let Ok(id) = SyscallFile::try_from(value) {
            Ok(Self::File(id))
        } else if let Ok(id) = SyscallIO::try_from(value) {
            Ok(Self::Io(id))
        } else if let Ok(id) = SyscallIPC::try_from(value) {
            Ok(Self::Ipc(id))
        } else if let Ok(id) = SyscallMM::try_from(value) {
            Ok(Self::Mm(id))
        } else if let Ok(id) = SyscallProc::try_from(value) {
            Ok(Self::Proc(id))
        } else if let Ok(id) = SyscallSocket::try_from(value) {
            Ok(Self::Socket(id))
        } else if let Ok(id) = SyscallTimer::try_from(value) {
            Ok(Self::Timer(id))
        } else {
            Err(value)
        }
    }
}
