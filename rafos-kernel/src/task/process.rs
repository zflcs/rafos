use core::{
    sync::atomic::AtomicI32, 
    future::Future,
    pin::Pin,
    task::{Poll, Context},
};

use super::*;
use asyncc::*;
use buddy_system_allocator::LockedHeap;
use config::{USER_HEAP_SIZE, USER_HEAP_PTR, PAGE_MASK};
/// This mod define `Process`
/// 

use spin::{Lazy, Mutex};
use alloc::{vec::Vec, sync::{Arc, Weak}, boxed::Box};
use crate::{mm::{MM, VMFlags}, fs::{File, FDManager}, KernelError};

use super::TaskState;

pub static IDLE_PROCESS: Lazy<Arc<Process>> = Lazy::new(|| Arc::new(Process::idle()));


pub struct Process {
    // immutable
    pub pid: PidHandle,
    // mutable
    pub executor: Option<usize>,
    pub allocator: Option<&'static LockedHeap<32>>,
    pub state: Mutex<TaskState>,
    pub mm: Mutex<MM>,
    pub parent: Mutex<Option<Weak<Process>>>,
    pub children: Mutex<Vec<Arc<Process>>>,
    pub exit_code: AtomicI32,
    pub fd_table: Mutex<FDManager>,
}

impl Process {
    pub fn idle() -> Self {
        Self {
            pid: PidHandle(IDLE_PID),
            executor: None,
            allocator: None,
            state: Mutex::new(TaskState::RUNNABLE),
            mm: Mutex::new(MM::new().unwrap()),
            parent: Mutex::new(None),
            children: Mutex::new(Vec::new()),
            exit_code: AtomicI32::new(0),
            fd_table: Mutex::new(FDManager::new())
        }
    }

    // pub fn new_kp() -> Result<TaskRef, KernelError> {
    //     let mut mm = crate::mm::new_kernel()?;
    //     let executor_size = core::mem::size_of::<Executor>();
    //     let start_va = mm.find_free_area(mm.start_brk, (executor_size + PAGE_SIZE - 1) & PAGE_MASK)?;
    //     let executor = mm.alloc_write_type(
    //         start_va, 
    //         VMFlags::READ | VMFlags::WRITE, 
    //         &Executor::new()
    //     )?;
    //     log::debug!("{:?}", mm);
    //     // TODO: initializing the main task and add to Executor
    //     // executor.spawn(fut, priority, task_type);
    //     let process = Self {
    //         pid: pid_alloc(),
    //         executor: Some(executor as *mut Executor as *mut usize as usize),
    //         allocator: None,
    //         state: Mutex::new(TaskState::RUNNABLE),
    //         mm: Mutex::new(mm),
    //         parent: Mutex::new(Some(Arc::downgrade(&IDLE_PROCESS))),
    //         children: Mutex::new(Vec::new()),
    //         exit_code: AtomicI32::new(0),
    //         fd_table: Mutex::new(FDManager::new())
    //     };
    //     let task_ref = Asyncc::spawn(Box::new(process), 0, TaskType::KernelProcess);
    //     Ok(task_ref)
    // }
    
    
    // // pub fn new() -> Result<TaskRef, KernelError> {
    // pub fn new(elf_data: &[u8]) -> Result<TaskRef, KernelError> {
    //     let mut mm = MM::new(false)?;
    //     loader::from_elf(elf_data, &mut mm)?;
    //     let heap_start = mm.start_brk + 0x80814000;
    //     let heap_end = heap_start + USER_HEAP_SIZE;

    //     mm.alloc_write_vma(None, heap_start, heap_end, VMFlags::READ | VMFlags::WRITE | VMFlags::USER)?;
    //     let allocator = LockedHeap::<32>::new();
    //     unsafe { allocator.lock().init(heap_start.0, USER_HEAP_SIZE) };
    //     let allocator = mm.alloc_write_type::<LockedHeap<32>>(
    //         USER_HEAP_PTR.into(), 
    //         VMFlags::READ | VMFlags::WRITE | VMFlags::USER, 
    //         &allocator
    //     )?;
    //     let executor_size = core::mem::size_of::<Executor>();
    //     let start_va = mm.find_free_area(mm.start_brk, (executor_size + PAGE_SIZE - 1) & PAGE_MASK)?;
    //     let executor = mm.alloc_write_type(
    //         start_va, 
    //         VMFlags::READ | VMFlags::WRITE | VMFlags::USER, 
    //         &Executor::new()
    //     )?;
    //     log::debug!("{:?}", mm);
    //     // TODO: initializing the main task and add to Executor
    //     // executor.spawn(fut, priority, task_type);
    //     let process = Self {
    //         pid: pid_alloc(),
    //         executor: Some(executor as *mut Executor as *mut usize as usize),
    //         allocator: Some(allocator),
    //         state: Mutex::new(TaskState::RUNNABLE),
    //         mm: Mutex::new(mm),
    //         parent: Mutex::new(Some(Arc::downgrade(&IDLE_PROCESS))),
    //         children: Mutex::new(Vec::new()),
    //         exit_code: AtomicI32::new(0),
    //         fd_table: Mutex::new(FDManager::new())
    //     };
    //     let task_ref = Asyncc::spawn(Box::new(process), 0, TaskType::Process);
    //     Ok(task_ref)
    // }
}

impl Future for Process {
    type Output = i32;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.state.lock().contains(TaskState::ZOMBIE) {
            Poll::Ready(self.exit_code.load(core::sync::atomic::Ordering::Relaxed))
        } else {
            let token = self.mm.lock().page_table.satp();
            let executor = self.executor.unwrap();
            Asyncc::set_args2(token, executor);
            log::debug!("into process token: {:#X}, executor: {:#X}", token, executor);
            Poll::Pending
        }
    }
}

