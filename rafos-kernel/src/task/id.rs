use spin::Lazy;
use kernel_sync::SpinLock;
use id_alloc::*;

pub static PID_ALLOCATOR: Lazy<SpinLock<RecycleAllocator>> = Lazy::new(|| SpinLock::new(RecycleAllocator::new(1)));


pub const IDLE_PID: usize = 0;

pub struct PidHandle(pub usize);

pub fn pid_alloc() -> PidHandle {
    PidHandle(PID_ALLOCATOR.lock().alloc())
}

impl Drop for PidHandle {
    fn drop(&mut self) {
        PID_ALLOCATOR.lock().dealloc(self.0);
    }
}
