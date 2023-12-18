use core::{task::{Context, Poll}, sync::atomic::Ordering, pin::Pin};

use alloc::boxed::Box;
/// This mod define the async controller, which implemented 
/// 
/// 

use asyncc::*;
use config::ASYNCC_ADDR;
use mmrv::AllocatedFrame;

use crate::frame_alloc;


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
        // 
        "0:addi sp, sp, -8",
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
        "beqz a0, 1f",
        "addi a0, a0, -1",
        "beqz a0, 1f",
        // Exception/Interrupt: save all register in stack
        "ld a0, 0(sp)",
        "addi sp, sp, 8",
        // "addi sp, sp, -8*34",

        "1:ld a0, 0(sp)",
        "addi sp, sp, 8",
        "call {handler}",
        "call {execute}",
        "j 0b",
        asyncc_addr = const ASYNCC_ADDR,
        handler = sym handler,
        execute = sym execute,
        options(noreturn),
    );
}

/// This function is the primary component of the controller.
/// The output value(`Option<TaskRef>`) can be divided into three categories:
///     1. In the same thread(stack).
///     2. In the same process(address space), but in different thread(stack).
///     3. In different process(address space).
/// The input arguments  are directly passed by registers.
/// These arguments may be the last task send to the next one. 
/// For example, if the last task is a process, it will pass `address space token`, and `executor pointer`.
/// So the handler can change the control flow to the target process address space.
/// 
#[link_section = ".text.trampoline"]
#[no_mangle]
fn handler() -> Option<TaskRef> {
    let cause = asyncc::Asyncc::cause();
    let task = match cause {
        Cause::Finish => {
            let executor = asyncc::Asyncc::get_executor();
            Asyncc::set_curr(None);
            // TODO: check the priority bitmap in kernel `Executor`
            executor.fetch()
        },
        Cause::Await => {
            let cur_task = asyncc::Asyncc::get_curr();
            // log::debug!("{:?}", cur_task);
            if let Some(cur_task) = cur_task {
                let task = unsafe { &*cur_task.as_ptr() };
                match task.task_type {
                    // if the current task is a process, it must go to user process address space.
                    TaskType::KernelProcess => {
                        log::debug!("need to change executor, satp");
                        let args = Asyncc::get_args2();
                        log::debug!("{:#X?}", args);
                        Asyncc::set_curr(None);
                        Asyncc::reset(args.a[1] as *const usize as _);
                        let satp = args.a[0];
                        let mut stack = AllocatedFrame::new(true).unwrap().start_address().value();
                        log::debug!("stack {:#X}", stack);
                        unsafe {
                            riscv::register::satp::write(satp);
                            riscv::asm::sfence_vma_all();
                            core::arch::asm!(
                                "mv sp, {stack}",
                                "j {entry}",
                                stack = in(reg) stack,
                                entry = sym asyncc_entry,
                            );
                        }
                    }
                    _ => todo!(),
                };
            }
            Asyncc::set_curr(None);
            // TODO: check the priority bitmap in kernel `Executor`
            let executor = asyncc::Asyncc::get_executor();
            executor.fetch()
        },
        _ => {
            todo!()
        }
    };
    // log::debug!("{:?}", task);
    task
}

/// This function need to be defined in kernel or user process. 
#[no_mangle]
pub fn execute(task_ref: Option<TaskRef>) {
    if let Some(task_ref) = task_ref {
        let executor = Asyncc::get_executor();
        executor.state.store(ExecutorState::Running as _, Ordering::Relaxed);
        unsafe {
            Asyncc::set_curr(Some(task_ref));
            let waker = asyncc::from_task(task_ref);
            let mut cx = Context::from_waker(&waker);
            let task = Task::from_ref(task_ref);
            task.state.store(TaskState::Running as _, Ordering::Relaxed);
            let fut = &mut *task.fut.as_ptr();
            let mut future = Pin::new_unchecked(fut.as_mut());
            match future.as_mut().poll(&mut cx) {
                Poll::Ready(_) => { 
                    Asyncc::set_cause(crate::Cause::Finish);
                },
                Poll::Pending => {
                    task.state.store(TaskState::Pending as _, Ordering::Relaxed);
                    Asyncc::set_cause(crate::Cause::Await);
                    let _task_ref = task.as_ref();
                },
            }
        }
    } else {
        let executor = asyncc::Asyncc::get_executor();
        if executor.state.load(Ordering::Relaxed) == ExecutorState::Ready as _ {
            // Asyncc::spawn(Box::new(crate::test::test()), 0, asyncc::TaskType::Other);
            executor.state.store(ExecutorState::Running as _, Ordering::Relaxed);
        }
    }
}
