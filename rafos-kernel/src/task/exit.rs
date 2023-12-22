use errno::Errno;
use mmrv::VirtAddr;
use syscall::*;

use crate::{cpu::{cpu, idle_ctx, do_yield}, task::{TaskState, TaskContext, __move_to_next}, write_user};

use super::Task;



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

/// Checks if a child satisfies the pid and options given by the calling process.
fn valid_child(pid: isize, options: WaitOptions, task: &Task) -> bool {
    if pid > 0 {
        if task.pid != pid as usize {
            return false;
        }
    }

    /*
     * Here we assume that all processes in the same process group.
     * Thus the calling process will wait for any process.
     */

    /*
     * Wait for all children (clone and not) if __WALL is set;
     * otherwise, wait for clone children *only* if __WCLONE is
     * set; otherwise, wait for non-clone children *only*.  (Note:
     * A "clone" child here is one that reports to its parent
     * using a signal other than SIGCHLD.)
     */
    if (task.exit_signal != SIGCHLD) ^ options.contains(WaitOptions::__WCLONE)
        && !options.contains(WaitOptions::__WALL)
    {
        return false;
    }

    true
}

/// A helper for [`syscall_interface::SyscallProc::wait4`].
pub fn do_wait(
    pid: isize,
    options: WaitOptions,
    _infop: usize,
    wstatus: *mut isize,
    _rusage: usize,
) -> SyscallResult {
    log::trace!("WAIT4 {} {:?} status=0x{:x}", pid, options, wstatus as usize);
    loop {
        let mut flag = false;
        let mut need_sched = false;
        let mut child: usize = 0;
        let curr = cpu().curr.as_ref().unwrap();
        for (index, task) in curr.children.lock().iter().enumerate() {
            if !valid_child(pid, options, &task) {
                continue;
            }
            // a valid child exists but current task needs to suspend
            need_sched = true;

            let state = task.state();
            if state == TaskState::STOPPED {
                todo!()
            } else {
                if state == TaskState::DEAD {
                    continue;
                }
                if state == TaskState::ZOMBIE {
                    if !options.contains(WaitOptions::WEXITED) {
                        continue;
                    }
                    // a child with changed state exists
                    flag = true;
                    child = index;
                    break;
                }
                if !options.contains(WaitOptions::WCONTINUED) {
                    continue;
                }
            }
        }
        if !flag {
            if options.contains(WaitOptions::WNONHANG) || !need_sched {
                log::trace!("{:?}", curr.children);
                return Err(Errno::ECHILD);
            }
            unsafe { do_yield() };
        } else {
            // reclaim resources
            let child = curr.children.lock().remove(child);

            // store status information
            if !wstatus.is_null() {
                let status = (child.inner().exit_code << 8) as i32;
                write_user!(curr.mm(), VirtAddr::from(wstatus as usize), status, i32)?;
            }

            return Ok(child.pid);
        }
    }
}
