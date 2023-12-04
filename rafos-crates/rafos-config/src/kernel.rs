


/// kernel stack size
pub const KERNEL_STACK_SIZE: usize = 0x4000;
/// kernel heap size
pub const KERNEL_HEAP_SIZE: usize = 0x80_0000;
/// the amount of cpu
pub const CPU_NUM: usize = 4;
/// the physical memory end
pub const MEMORY_END: usize = 0x84000000;
/// page size: 4K
pub const PAGE_SIZE: usize = 0x1000;
///
pub const PAGE_SIZE_BITS: usize = 0xc;
/// the base address of trampoline
pub const TRAMPOLINE: usize = usize::MAX - PAGE_SIZE + 1;
/// the trap context of user thread 0
pub const TRAP_CONTEXT: usize = TRAMPOLINE - PAGE_SIZE;

/// The highest virtual address of the low 256 GB in SV39.
// pub const LOW_MAX_VA: usize = 0x0000_003F_FFFF_FFFF;
pub const LOW_MAX_VA: usize = 0xFFFF_FFFF;

/// User maximum pages
pub const USER_MAX_PAGES: usize = (LOW_MAX_VA + 1) >> PAGE_SIZE_BITS;

/// Maximum virtual memory areas in an address space
pub const MAX_MAP_COUNT: usize = 256;

/// 
pub const USER_STACK_BASE: usize = LOW_MAX_VA + 1;

#[cfg(feature = "board_qemu")]
/// the clock frequency in qemu
pub const CLOCK_FREQ: usize = 12500000;
