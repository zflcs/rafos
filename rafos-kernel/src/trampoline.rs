
use core::{task::{Context, Poll}, sync::atomic::Ordering, pin::Pin};
use asyncc::*;


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
fn handler() {
    // let cause = asyncc::Asyncc::cause();
    // let task = match cause {
    //     Cause::Finish => {
    //         let executor = asyncc::Asyncc::get_executor();
    //         Asyncc::set_curr(None);
    //         // TODO: check the priority bitmap in kernel `Executor`
    //         executor.fetch()
    //     },
    //     Cause::Await => {
    //         let cur_task = asyncc::Asyncc::get_curr();
    //         // log::debug!("{:?}", cur_task);
    //         if let Some(cur_task) = cur_task {
    //             let task = unsafe { &*cur_task.as_ptr() };
    //             match task.task_type {
    //                 // if the current task is a process, it must go to user process address space.
    //                 TaskType::KernelProcess => {
    //                     log::debug!("need to change executor, satp");
    //                     let args = Asyncc::get_args2();
    //                     log::debug!("{:#X?}", args);
    //                     Asyncc::set_curr(None);
    //                     Asyncc::reset(args.a[1] as *const usize as _);
    //                     let satp = args.a[0];
    //                     let mut stack = AllocatedFrame::new(true).unwrap().start_address().value();
    //                     log::debug!("stack {:#X}", stack);
    //                     unsafe {
    //                         riscv::register::satp::write(satp);
    //                         riscv::asm::sfence_vma_all();
    //                         core::arch::asm!(
    //                             "mv sp, {stack}",
    //                             "j {entry}",
    //                             stack = in(reg) stack,
    //                             entry = sym asyncc_entry,
    //                         );
    //                     }
    //                 }
    //                 _ => todo!(),
    //             };
    //         }
    //         Asyncc::set_curr(None);
    //         // TODO: check the priority bitmap in kernel `Executor`
    //         let executor = asyncc::Asyncc::get_executor();
    //         executor.fetch()
    //     },
    //     _ => {
    //         todo!()
    //     }
    // };
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
