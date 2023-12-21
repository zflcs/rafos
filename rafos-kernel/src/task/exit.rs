use errno::Errno;
use mmrv::VirtAddr;
use syscall::SyscallResult;

use crate::{cpu::{cpu, idle_ctx, do_yield}, task::{TaskState, TaskContext, __move_to_next}, write_user};



/// Current task exits. Run next task.
///
/// # Safety
///
/// Unsafe context switch will be called in this function.
pub unsafe fn do_exit(exit_code: isize) {
    let curr = cpu().curr.as_ref().unwrap();
    let _curr_ctx = {
        curr.inner().exit_code = exit_code;
        *curr.state.lock() = TaskState::ZOMBIE;
        &curr.inner().context as *const TaskContext
    };
    log::trace!("{:?} exited with code {}", curr, exit_code);

    __move_to_next(idle_ctx());
}


/// A helper for [`syscall_interface::SyscallProc::wait4`].
pub fn do_wait(pid: isize, exit_code_ptr: usize) -> SyscallResult {
    loop {
        log::trace!("WAIT4 {}", pid);
        let curr = cpu().curr.as_ref().unwrap();
        if curr.children
            .lock()
            .iter()
            .find(|child| pid == -1 || pid as usize == child.tid.0)
            .is_none() {
                log::trace!("no child process");
            return Err(Errno::ESRCH);
        };
        let mut children = curr.children.lock();
        let pair = children
            .iter()
            .enumerate()
            .find(|(_, child)| 
                child.state() == TaskState::ZOMBIE && (pid == -1 || pid as usize == child.tid.0)
        );
        if let Some((idx, _)) = pair {
            let child = children.remove(idx);
            let pid = child.tid.0;
            let exit_code = child.inner().exit_code;
            write_user!(curr.mm(), VirtAddr::from(exit_code_ptr), exit_code, i32)?;
            log::trace!("wait {}, exit_code_ptr {:#X} exit_code: {}", pid, exit_code_ptr, exit_code);
            return Ok(pid);
        } else {
            unsafe { 
                do_yield();
                log::trace!("here");
            }
        }
    }
}

///
pub fn do_wait_tid(tid: usize) -> SyscallResult {
    loop {
        log::trace!("wait_tid {}", tid);
        let curr = cpu().curr.as_ref().unwrap();
        let pid = curr.pid;
        if curr.children
            .lock()
            .iter()
            .find(|child| tid == child.tid.0 && pid == child.pid)
            .is_none() {
                log::trace!("no such process");
            return Err(Errno::ESRCH);
        };
        let mut children = curr.children.lock();
        let pair = children
            .iter()
            .enumerate()
            .find(|(_, child)| 
                child.state() == TaskState::ZOMBIE && (tid == child.tid.0 && pid == child.pid)
        );
        if let Some((idx, _)) = pair {
            let child = children.remove(idx);
            let exit_code = child.inner().exit_code;
            return Ok(exit_code as _);
        } else {
            unsafe { 
                do_yield();
                log::trace!("here");
            }
        }
    }
}