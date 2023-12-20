
use config::{KERNEL_STACK_SIZE, ADDR_ALIGN, KERNEL_STACK_PAGES};
use mmrv::*;
use crate::{KernelResult, KernelError};

/// A wrapper for kernel stack.
pub struct KernelStack(AllocatedFrameRange);

impl KernelStack {
    /// Creates a new kernel stack.
    pub fn new() -> KernelResult<Self> {
        Ok(Self(
            AllocatedFrameRange::new(KERNEL_STACK_PAGES, true)
                .map_err(|_| KernelError::FrameAllocFailed)?,
        ))
    }

    /// Returns base address of [`KernelStack`].
    pub fn base(&self) -> usize {
        self.0.start_address().value() + KERNEL_STACK_SIZE - ADDR_ALIGN
    }

    /// Returns top address of [`KernelStack`].
    pub fn top(&self) -> usize {
        self.0.start_address().value()
    }
}