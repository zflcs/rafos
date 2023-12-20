


/// kernel stack size
pub const KERNEL_STACK_SIZE: usize = 0x4000;

/// Kernel stack pages
pub const KERNEL_STACK_PAGES: usize = KERNEL_STACK_SIZE >> PAGE_SIZE_BITS;

/// Address alignment
pub const ADDR_ALIGN: usize = core::mem::size_of::<usize>();

/// kernel heap size
pub const KERNEL_HEAP_SIZE: usize = 0x80_0000;

/// Relocatable file address
pub const ELF_BASE_RELOCATE: usize = 0x8000_0000;

/// the amount of cpu
pub const CPU_NUM: usize = 4;
/// the physical memory end
pub const MEMORY_END: usize = 0x84000000;

/// page size: 4K
const PAGE_SIZE: usize = 0x1000;
/// page size: 4K
pub const PAGE_MASK: usize = !0x0FFF;
///
pub const PAGE_SIZE_BITS: usize = 0xc;
/// the base address of trampoline
pub const TRAMPOLINE: usize = usize::MAX - PAGE_SIZE + 1;
/// the trap context of user thread 0
pub const TRAP_CONTEXT: usize = TRAMPOLINE - PAGE_SIZE;

/// The highest virtual address of the low 256 GB in SV39.
// pub const LOW_MAX_VA: usize = 0x0000_003F_FFFF_FFFF;
const LOW_MAX_VA: usize = 0xFFFF_FFFF;

/// User maximum pages
pub const USER_MAX_PAGES: usize = (LOW_MAX_VA + 1) >> PAGE_SIZE_BITS;

/// Maximum virtual memory areas in an address space
pub const MAX_MAP_COUNT: usize = 256;

///
pub const USER_HEAP_PTR: usize = LOW_MAX_VA - PAGE_SIZE * 2 + 1;

/// 
pub const USER_STACK_BASE: usize = LOW_MAX_VA + 1;

/// Default maximum file descriptor limit.
pub const DEFAULT_FD_LIMIT: usize = 0x100;

#[cfg(feature = "board_qemu")]
/// the clock frequency in qemu
pub const CLOCK_FREQ: usize = 12500000;


/// The Asyncc Controller base address
pub const ASYNCC_ADDR: usize = 0xf000000;
///
pub const ASYNCC_LEN: usize = 0x10000;

/// The Executor base address
pub const EXECUTOR_BASE_ADDR: usize = 0x8600_0000;

/// MMIO
#[cfg(feature = "board_qemu")]
pub const MMIO: &[(usize, usize)] = &[
    (0x1000_6000,   0x3000),        // virtio
    (ASYNCC_ADDR,   0x1000000)      // lite executor
];

#[cfg(feature = "board_axu15eg")]
pub const MMIO: &[(usize, usize)] = &[
    (0x6000_0000,   0x200_0000),        // virtio
    (0xf000000,     0x1000000)      // lite executor
];