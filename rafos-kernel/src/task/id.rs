use spin::Lazy;
use kernel_sync::SpinLock;
use id_alloc::*;

pub static PID_ALLOCATOR: Lazy<SpinLock<RecycleAllocator>> = Lazy::new(|| SpinLock::new(RecycleAllocator::new(1)));


pub const IDLE_PID: usize = 0;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TidHandle(pub usize);

impl TidHandle {
    pub fn new() -> Self {
        Self(PID_ALLOCATOR.lock().alloc())
    }
}

impl Drop for TidHandle {
    fn drop(&mut self) {
        PID_ALLOCATOR.lock().dealloc(self.0);
    }
}
