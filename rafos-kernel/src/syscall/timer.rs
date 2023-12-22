use errno::Errno;
use mmrv::*;
use time::{TimeSpec, NSEC_PER_SEC, Instant, Duration};
use crate::{cpu::{cpu, do_yield}, read_user, write_user};
use syscall::*;

use super::SyscallImpl;

impl SyscallTimerTrait for SyscallImpl {
    fn sys_nano_sleep(rqtp: *const time::TimeSpec, rmtp: *mut time::TimeSpec) -> SyscallResult {
        let req_addr = VirtAddr::from(rqtp as usize);
        let mut req = TimeSpec::new(0.0);
        read_user!(cpu().curr.as_ref().unwrap().mm(), req_addr, req, TimeSpec)?;

        if req.tv_nsec >= NSEC_PER_SEC {
            return Err(Errno::EINVAL);
        }
        let start = Instant::now();
        let duration = Duration::from_nanos((req.tv_sec * NSEC_PER_SEC + req.tv_nsec) as u64);
        while start.elapsed() < duration {
            unsafe { do_yield() };
        }

        if !rmtp.is_null() {
            let rem_addr = VirtAddr::from(rmtp as usize);
            let rem = TimeSpec::new(0.0);
            write_user!(cpu().curr.as_ref().unwrap().mm(), rem_addr, rem, TimeSpec)?;
        }
        Ok(0)
    }
}

