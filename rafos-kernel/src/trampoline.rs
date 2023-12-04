/// This mod define the async controller, which implemented 
/// 
/// 

use asyncc::*;
use config::ASYNCC_ADDR;


/// The reasons why control flow turn to this function
/// - After kernel init, it jump to this function
/// - A task has been polled, no matter whether it is finished
/// - Interrupt
/// - Exception
/// 
#[link_section = ".text.trampoline"]
#[no_mangle]
#[naked]
pub unsafe extern "C" fn asyncc_entry() {
    core::arch::asm!(
        "auipc ra, 0",
        // 
        "addi sp, sp, -8",
        "sd a0, 0(sp)",
        // a0 => Asyncc
        "li a0, {asyncc_addr}",
        // a0 => cause register
        "addi a0, a0, 4",
        "lw a0, 0(a0)",
        "srli a0, a0, 30",
        "andi a0, a0, 3",
        // finish/await: the context don't need to save
        // so just jump to handler function
        "beqz a0, {handler}",
        "addi a0, a0, -1",
        "beqz a0, {handler}",
        // Exception/Interrupt: save all register in stack
        "ld a0, 0(sp)",
        "addi sp, sp, 8",
        // "addi sp, sp, -8*34",

        "j {handler}",
        asyncc_addr = const ASYNCC_ADDR,
        handler = sym handler,
        options(noreturn),
    );
}

#[link_section = ".text.trampoline"]
#[no_mangle]
fn handler() {
    let cause = asyncc::Asyncc::cause();
    match cause {
        Cause::Finish => {
            let executor = asyncc::Asyncc::get_executor();
            if let Some(task_ref) = executor.fetch(0) {
                if let Some(task_ref) = execute(task_ref) {
                    if (unsafe { &*task_ref.as_ptr() }).task_type == TaskType::KernelSche {
                        executor.wake_task_from_ref(task_ref);
                    }
                }
            }
            log::debug!("task finish");
        },
        Cause::Await => {
            let executor = asyncc::Asyncc::get_executor();
            if let Some(task_ref) = executor.fetch(0) {
                if let Some(task_ref) = execute(task_ref) {
                    if (unsafe { &*task_ref.as_ptr() }).task_type == TaskType::KernelSche {
                        executor.wake_task_from_ref(task_ref);
                    }
                }
            }
            log::debug!("task pending");
        },
        Cause::Intr(_) => {
            log::debug!("intr occur");
        },
        Cause::Exception(_) => {
            log::debug!("exception occur");
        },
    }
}

