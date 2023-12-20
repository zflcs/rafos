
use xmas_elf::{
    header,
    program::{self, SegmentData},
    ElfFile,
};
use mmrv::*;
use crate::{
    KernelError, KernelResult,
    mm::{MM, VMFlags}
};
use config::*;


/// Create address space from elf.
pub fn from_elf(elf_data: &[u8], mm: &mut MM) -> KernelResult<VirtAddr> {
    let elf = ElfFile::new(elf_data).unwrap();
    let elf_hdr = elf.header;

    // Check elf type
    if (elf_hdr.pt2.type_().as_type() != header::Type::Executable
        && elf_hdr.pt2.type_().as_type() != header::Type::SharedObject)
        // 64-bit format
        || elf_hdr.pt1.class() != header::Class::SixtyFour
        // 'E', 'L', 'F'
        || elf_hdr.pt1.magic != [0x7f, 0x45, 0x4c, 0x46]
        // RISC-V
        || elf_hdr.pt2.machine().as_machine() != header::Machine::RISC_V
    {
        return Err(KernelError::ELFInvalidHeader);
    }

    // Dynamic address
    let mut dyn_base = 0;
    let elf_base_va = if let Some(phdr) = elf
        .program_iter()
        .find(|phdr| phdr.get_type() == Ok(program::Type::Load) && phdr.offset() == 0)
    {
        let phdr_va = phdr.virtual_addr() as usize;
        if phdr_va != 0 {
            phdr_va
        } else {
            // If the first segment starts at 0, we need to put it at a higher address
            // to avoid conflicts with user programs.
            dyn_base = ELF_BASE_RELOCATE;
            ELF_BASE_RELOCATE
        }
    } else {
        0
    };

    // Load program header
    let mut max_page = Page::from(0);
    for phdr in elf.program_iter() {
        match phdr.get_type().unwrap() {
            program::Type::Load => {
                let start_va: VirtAddr = (phdr.virtual_addr() as usize).into();
                let end_va: VirtAddr = ((phdr.virtual_addr() + phdr.mem_size()) as usize).into();
                max_page = Page::floor(end_va - 1) + 1;

                // Map flags
                let mut map_flags = VMFlags::USER;
                let phdr_flags = phdr.flags();
                if phdr_flags.is_read() {
                    map_flags |= VMFlags::READ;
                }
                if phdr_flags.is_write() {
                    map_flags |= VMFlags::WRITE;
                }
                if phdr_flags.is_execute() {
                    map_flags |= VMFlags::EXEC;
                }

                // Allocate a new virtual memory area
                let data = match phdr.get_data(&elf).unwrap() {
                    SegmentData::Undefined(data) => data,
                    _ => return Err(KernelError::ELFInvalidSegment),
                };
                
                // Address may not be aligned.
                mm.alloc_write_vma(
                    Some(data),
                    start_va + dyn_base,
                    end_va + dyn_base,
                    map_flags,
                )?;
            }
            program::Type::Interp => {
                // let data = match phdr.get_data(&elf).unwrap() {
                //     SegmentData::Undefined(data) => data,
                //     _ => return Err(KernelError::ELFInvalidSegment),
                // };
                // let path = unsafe {raw_ptr_to}
            }
            _ => {}
        };
    }

    // .rela.dyn

    // .rela.plt

    // Set brk location
    mm.start_brk = max_page.start_address() + dyn_base;
    mm.brk = mm.start_brk;

    // Set user entry
    mm.entry = VirtAddr::from(elf_hdr.pt2.entry_point() as usize) + dyn_base;

    // Initialize user stack
    let ustack_base = USER_STACK_BASE - ADDR_ALIGN;
    let ustack_top = USER_STACK_BASE - USER_STACK_SIZE;
    mm.alloc_write_vma(
        None,
        ustack_top.into(),
        ustack_base.into(),
        VMFlags::READ | VMFlags::WRITE | VMFlags::USER,
    )?;
    let vsp = VirtAddr::from(ustack_base);
    Ok(vsp)
}