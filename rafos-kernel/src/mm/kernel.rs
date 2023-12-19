use spin::Lazy;
use kernel_sync::SpinLock;
use config::MEMORY_END;

use super::*;

extern "C" {
    fn stext();
    fn etext();
    fn srodata();
    fn erodata();
    fn sdata();
    fn edata();
    fn sbss_with_stack();
    fn ebss();
    fn ekernel();
}

pub static KERNEL_SPACE: Lazy<Arc<SpinLock<MM>>> = Lazy::new(|| Arc::new(SpinLock::new(new_kernel().unwrap())));

pub fn kernel_token() -> usize {
    KERNEL_SPACE.lock().page_table.satp()
}

///
pub fn kernel_activate() {
    let satp = kernel_token();
    unsafe {
        riscv::register::satp::write(satp);
        riscv::asm::sfence_vma_all();
    }
}

/// Without kernel stacks.
fn new_kernel() -> Result<MM, KernelError> {
    let mut mm = MM::new(true)?;
    // Map kernel .text section
    mm.alloc_write_vma(
        None,
        (stext as usize).into(),
        (etext as usize).into(),
        VMFlags::READ | VMFlags::EXEC | VMFlags::IDENTICAL,
    )?;

    // Map kernel .rodata section
    mm.alloc_write_vma(
        None,
        (srodata as usize).into(),
        (erodata as usize).into(),
        VMFlags::READ | VMFlags::IDENTICAL,
    )?;

    // Map kernel .data section
    mm.alloc_write_vma(
        None,
        (sdata as usize).into(),
        (edata as usize).into(),
        VMFlags::READ | VMFlags::WRITE | VMFlags::IDENTICAL,
    )?;

    // Map kernel .bss section
    mm.alloc_write_vma(
        None,
        (sbss_with_stack as usize).into(),
        (ebss as usize).into(),
        VMFlags::READ | VMFlags::WRITE | VMFlags::IDENTICAL,
    )?;

    // Physical memory area
    mm.alloc_write_vma(
        None,
        (ekernel as usize).into(),
        MEMORY_END.into(),
        VMFlags::READ | VMFlags::WRITE | VMFlags::IDENTICAL,
    )?;

    // plic
    mm.alloc_write_vma(
        None,
        0xc00_0000.into(),
        0x1000_0000.into(),
        VMFlags::READ | VMFlags::WRITE | VMFlags::IDENTICAL,
    )?;


    for (start, len) in MMIO {
        mm.alloc_write_vma(
            None,
            (*start).into(),
            ((*start) + (*len)).into(),
            VMFlags::READ | VMFlags::WRITE | VMFlags::IDENTICAL,
        )?;
    }

    mm.start_brk = MEMORY_END.into();
    unsafe { core::arch::asm!("fence.i") }
    log::debug!("{:?}", mm);
    Ok(mm)
}