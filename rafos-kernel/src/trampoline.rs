


use mmrv::VirtAddr;
use riscv::register::{scause::*, utvec::TrapMode, *};
use crate::{task::*, cpu::*, syscall::*, mm::{do_handle_page_fault, VMFlags}};
use config::TRAMPOLINE;
use crate::KernelError;

/// Set user trap entry.
fn set_user_trap() {
    unsafe { stvec::write(TRAMPOLINE as usize, TrapMode::Direct) };
}

// pub fn enable_timer_intr() {
//     unsafe {
//         sie::set_stimer();
//         // sstatus::set_sie();
//     }
// }

/// User trap handler manages the task according to the cause:
///
/// 1. Calls syscall dispatcher and handler.
/// 2. Handles page fault caused by Instruction Fetch, Load or Store.
#[no_mangle]
pub fn user_trap_handler() -> ! {
    let scause = scause::read();
    let sstatus = sstatus::read();
    let stval = stval::read();
    let sepc = sepc::read();
    // Only handle user trap
    assert!(sstatus.spp() == sstatus::SPP::User);

    // Handle user trap with detailed cause
    let show_trapframe = |tf: &TrapFrame| {
        println!("{:#X?}", tf);
    };
    let trap_info = || {
        trace!(
            "[U] {:X?}, {:X?}, stval={:#X}, sepc={:#X} {:?}",
            scause.cause(),
            sstatus,
            stval,
            sepc,
            cpu().curr.as_ref().unwrap(),
        )
    };
    let fatal_info = |err: KernelError| {
        trace!("[U] Fatal exception {:#?}", err);
    };

    match scause.cause() {
        Trap::Exception(Exception::UserEnvCall) => {
            log::trace!("user env call");
            // pc + 4
            let curr = cpu().curr.as_ref().unwrap();
            let trapframe = curr.trapframe();
            trapframe.next_epc();
            match syscall(trapframe.syscall_args().unwrap()) {
                Ok(ret) => trapframe.set_a0(ret),
                Err(errno) => {
                    trace!("{:#?} {:#?}", trapframe.syscall_args().unwrap().0, errno);
                    trapframe.set_a0(-isize::from(errno) as usize)
                }
            };
        }
        Trap::Exception(Exception::StorePageFault) => {
            let curr = cpu().curr.as_ref().unwrap();
            let mut curr_mm = curr.mm();
            trap_info();
            if let Err(err) = do_handle_page_fault(
                &mut curr_mm,
                VirtAddr::from(stval),
                VMFlags::USER | VMFlags::WRITE,
            ) {
                fatal_info(err);
                unsafe { do_exit(-1) };
            }
        }
        _ => {
            let curr = cpu().curr.as_ref().unwrap();
            show_trapframe(curr.trapframe());
            trap_info();
            // unsafe { do_exit(-1) };
            panic!("not supported");
        }
    }
    user_trap_return();
}

/// Something prepared before `sret` back to user:
///
/// 1. Set `stvec` to user trap entry again.
/// 2. Jump to raw assembly code, passing the address of trapframe and `satp`.
///
/// # DEAD LOCK
///
/// This function acquires a reference and the lock of address space metadata of
/// current task. We must drop them before changing the control flow without unwinding.
#[no_mangle]
pub fn user_trap_return() -> ! {
    let (satp, trapframe_base, userret) = {
        let curr = cpu().curr.as_ref().unwrap();
        let curr_mm = curr.mm();
        (
            curr_mm.page_table.satp(),
            trapframe_base(curr.tid.0),
            user_ret as usize - user_vec as usize + TRAMPOLINE,
        )
    };
    log::trace!("satp:{:#X}\ntrapframe_base:{:#X}\nuserret:{:#X}", satp, trapframe_base, userret);
    set_user_trap();

    unsafe {
        core::arch::asm!(
            "jr {userret}",
            userret = in(reg) userret,
            in("a0") trapframe_base,
            in("a1") satp,
            options(noreturn)
        );
    }
}



/// The reasons why control flow turn to this function
/// - After kernel init, it jump to this function
/// - A task has been polled, no matter whether it is finished
/// 
/// There is no `Interrupt` or `Exeception` and the `Task` is stackless coroutine.
/// Once the control flow is here, it means that the stack can be cleaned. 
/// Besides, there is no need to save the general registers.
/// 
#[link_section = ".text.trampoline"]
#[no_mangle]
#[naked]
pub unsafe extern "C" fn user_vec() {
    core::arch::asm!(
        ".align 2",
        // Now sp points to user trapframe, and sscratch points to user stack.
        "csrrw sp, sscratch, sp",
        // Save user registers in trapframe.
        "
        sd ra, 40(sp)
        sd gp, 56(sp)
        sd tp, 64(sp)
        sd t0, 72(sp)
        sd t1, 80(sp)
        sd t2, 88(sp)
        sd s0, 96(sp)
        sd s1, 104(sp)
        sd a0, 112(sp)
        sd a1, 120(sp)
        sd a2, 128(sp)
        sd a3, 136(sp)
        sd a4, 144(sp)
        sd a5, 152(sp)
        sd a6, 160(sp)
        sd a7, 168(sp)
        sd s2, 176(sp)
        sd s3, 184(sp)
        sd s4, 192(sp)
        sd s5, 200(sp)
        sd s6, 208(sp)
        sd s7, 216(sp)
        sd s8, 224(sp)
        sd s9, 232(sp)
        sd s10, 240(sp)
        sd s11, 248(sp)
        sd t3, 256(sp)
        sd t4, 264(sp)
        sd t5, 272(sp)
        sd t6, 280(sp)
        csrr t0, sscratch
        sd t0, 48(sp)
        ",
        // Save sepc and sstatus
        "
        csrr t0, sepc
        csrr t1, sstatus
        sd t0, 24(sp)
        sd t1, 32(sp)
        ",
        // Load the virtual address of trap handler
        "ld t0, 16(sp)",
        // Load the kernel page table root address
        "ld t1, 0(sp)",
        // Load cpu id
        "ld tp, 288(sp)",
        // Initialize kernel stack pointer
        "ld sp, 8(sp)",
        // Change to the kernel page table root
        "csrw satp, t1",
        // Flush all satle TLB entries
        "sfence.vma zero, zero",
        // Jump to trap handler
        "jr t0",
        options(noreturn),
    )
}

#[link_section = ".text.trampoline"]
#[no_mangle]
#[naked]
pub unsafe extern "C" fn user_ret() {
    core::arch::asm!(
        "
        .align 2
        csrw satp, a1
        sfence.vma zero, zero
        ",
        // Now sscratch agiain points to user trapframe
        "csrw sscratch, a0",
        // Save cpu id
        "sd tp, 288(a0)",
        // Restore sepc and sstatus
        "
        ld t0, 24(a0)
        ld t1, 32(a0)
        csrw sepc, t0
        csrw sstatus, t1
        ",
        // Restore user registers
        "
        ld ra, 40(a0)
        ld sp, 48(a0)
        ld gp, 56(a0)
        ld tp, 64(a0)
        ld t0, 72(a0)
        ld t1, 80(a0)
        ld t2, 88(a0)
        ld s0, 96(a0)
        ld s1, 104(a0)
        ld a1, 120(a0)
        ld a2, 128(a0)
        ld a3, 136(a0)
        ld a4, 144(a0)
        ld a5, 152(a0)
        ld a6, 160(a0)
        ld a7, 168(a0)
        ld s2, 176(a0)
        ld s3, 184(a0)
        ld s4, 192(a0)
        ld s5, 200(a0)
        ld s6, 208(a0)
        ld s7, 216(a0)
        ld s8, 224(a0)
        ld s9, 232(a0)
        ld s10, 240(a0)
        ld s11, 248(a0)
        ld t3, 256(a0)
        ld t4, 264(a0)
        ld t5, 272(a0)
        ld t6, 280(a0)
        ",
        
        // Finally restore a0
        "ld a0, 112(a0)",
        // Return to user context
        "sret",
        options(noreturn),
    )
}
