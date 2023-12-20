use config::TRAMPOLINE;
use mmrv::*;
use riscv::register::sstatus::{Sstatus, set_spp, SPP, self};
use syscall::SyscallId;
use crate::{KernelResult, KernelError, mm::MM, syscall::*};

/// Trap frame tracker
#[repr(transparent)]
pub struct TrapFrameTracker(pub AllocatedFrame);

impl Drop for TrapFrameTracker {
    fn drop(&mut self) {
        log::trace!("Drop {:?}", self.0)
    }
}

/// Returns trapframe base of the task in the address space by task identification.
///
/// Trapframes are located right below the Trampoline in each address space.
pub fn trapframe_base(tid: usize) -> usize {
    TRAMPOLINE - PAGE_SIZE - tid * PAGE_SIZE
}

/// Initialize trapframe
pub fn init_trapframe(mm: &mut MM, tid: usize) -> KernelResult<TrapFrameTracker> {
    let trapframe = AllocatedFrame::new(true).map_err(|_| KernelError::FrameAllocFailed)?;
    let trapframe_va: VirtAddr = trapframe_base(tid).into();
    mm.page_table
        .map(
            Page::from(trapframe_va),
            trapframe.clone(),
            PTEFlags::READABLE | PTEFlags::WRITABLE | PTEFlags::VALID,
        )
        .map_err(|_| KernelError::PageTableInvalid)?;
    Ok(TrapFrameTracker(trapframe))
}

/// User context is saved in trapframe by trap handler in trampoline.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct TrapFrame {
    /// Kernel page table root
    kernel_satp: usize,
    /// Kernel stack pointer
    kernel_sp: usize,
    /// Trap handler address
    trap_handler: usize,
    /// User program counter
    user_epc: usize,
    /// User status
    user_status: Sstatus,
    /// Saved global registers (arch dependent)
    /// No need to save x0 (wired to zero)
    user_regs: [usize; 31],
    /// Saved hartid
    cpu_id: usize,
}

impl TrapFrame {
    /// Create a new trap frame with user stack pointer.
    pub fn new(
        kernel_satp: usize,
        kernel_sp: usize,
        trap_handler: usize,
        user_epc: usize,
        user_sp: usize,
    ) -> Self {
        unsafe { set_spp(SPP::User) };
        let mut trapframe = Self {
            kernel_satp,
            kernel_sp,
            trap_handler,
            user_epc,
            user_status: sstatus::read(),
            user_regs: [0; 31],
            cpu_id: usize::MAX,
        };
        trapframe.user_regs[1] = user_sp;
        trapframe
    }

    /// Copies from the old one when we clone a task and initialize its trap frame.
    pub fn copy_from(
        &mut self,
        orig: &TrapFrame,
        stack: usize,
        kstack: usize,
    ) {
        *self = *orig;
        // Sets new kernel stack
        self.kernel_sp = kstack;
        // Child task returns zero
        self.set_a0(0);
        // Set stack pointer
        if stack != 0 {
            self.set_sp(stack);
        }
    }

    /// Gets syscall number.
    pub fn syscall_no(&self) -> usize {
        self.user_regs[16]
    }

    pub fn syscall_args(&self) -> KernelResult<SyscallArgs> {
        Ok(SyscallArgs(
            SyscallId::try_from(self.user_regs[16])
                .map_err(|no| KernelError::SyscallUnsupported(no))?,
            [
                self.user_regs[9],  // x10
                self.user_regs[10], // x11
                self.user_regs[11], // x12
                self.user_regs[12], // x13
                self.user_regs[13], // x14
                self.user_regs[14], // x15
            ],
        ))
    }

    /// Step to next instruction after the trap instruction.
    pub fn next_epc(&mut self) {
        self.user_epc += 4;
    }

    /// Returns mutable reference of a trapframe
    pub fn from(pa: PhysAddr) -> &'static mut TrapFrame {
        unsafe { (pa.value() as *mut TrapFrame).as_mut().unwrap() }
    }

    /// Set return errno or value after an syscall.
    pub fn set_a0(&mut self, a0: usize) {
        self.user_regs[9] = a0;
    }

    /// Set stack pointer while cloning task
    pub fn set_sp(&mut self, sp: usize) {
        self.user_regs[1] = sp;
    }

    /// Set tp while cloning task with tls
    pub fn set_tp(&mut self, tp: usize) {
        self.user_regs[3] = tp;
    }
}

