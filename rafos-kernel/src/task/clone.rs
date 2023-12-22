use super::*;
use syscall::{CloneFlags, SigPending, SigSet, SIGNONE, sigvalid};
use errno::Errno;

///
pub fn do_clone(entry: usize, flags: CloneFlags, stack: usize, arg: *const usize, ptid: VirtAddr, tls: usize, ctid: VirtAddr) -> SyscallResult {
    let curr = cpu().curr.as_ref().unwrap();
    log::trace!("CLONE {:?} {:?}", curr, flags);
    if flags.contains(CloneFlags::CLONE_NEWNS | CloneFlags::CLONE_FS) {
        return Err(Errno::EINVAL);
    }
    if flags.contains(CloneFlags::CLONE_THREAD) && !flags.contains(CloneFlags::CLONE_SIGHAND) {
        return Err(Errno::EINVAL);
    }
    if flags.contains(CloneFlags::CLONE_SIGHAND) && !flags.contains(CloneFlags::CLONE_VM) {
        return Err(Errno::EINVAL);
    }
    let mm = if flags.contains(CloneFlags::CLONE_VM) {
        curr.inner().mm.clone()
    } else {
        Arc::new(SpinLock::new(curr.mm().clone()?))
    };
    let files = if flags.contains(CloneFlags::CLONE_FILES) {
        curr.inner().files.clone()
    } else {
        Arc::new(SpinLock::new(curr.files().clone()))
    };
    let fs_info = if flags.contains(CloneFlags::CLONE_FS) {
        curr.fs_info.clone()
    } else {
        let orig = curr.fs_info.lock();
        Arc::new(SpinLock::new(orig.clone()))
    };
    let sig_actions = if flags.intersects(CloneFlags::CLONE_SIGHAND | CloneFlags::CLONE_THREAD) {
        curr.sig_actions.clone()
    } else {
        let orig = curr.sig_actions.lock();
        Arc::new(SpinLock::new(orig.clone()))
    };
    let parent = if flags.intersects(CloneFlags::CLONE_PARENT | CloneFlags::CLONE_THREAD) {
        SpinLock::new(curr.parent.lock().clone())
    } else {
        SpinLock::new(Some(Arc::downgrade(&curr)))
    };
    let set_child_tid = if flags.contains(CloneFlags::CLONE_CHILD_SETTID) {
        ctid.value()
    } else {
        0
    };
    let clear_child_tid = if flags.contains(CloneFlags::CLONE_CHILD_CLEARTID) {
        ctid.value()
    } else {
        0
    };
    
    // New kernel stack
    let kstack = KernelStack::new()?;
    let tid = TidHandle::new();
    let tid_num = tid.0;
    let kstack_base = kstack.base();
    
    // Init trapframe
    let trapframe_tracker = {
        let mut mm = mm.lock();
        let trapframe_tracker = init_trapframe(&mut mm, tid_num)?;
        let trapframe = TrapFrame::from(trapframe_tracker.0.start_address());
        trapframe.copy_from(curr.trapframe(), flags, stack, tls, kstack_base);
        if entry != 0 {
            log::trace!("entry {:#X}, stack {:#X}", entry, stack);
            trapframe.set_epc(entry);
        }
        trapframe.set_a0(arg as _);
        trapframe_tracker
    };
    let pid = if flags.contains(CloneFlags::CLONE_THREAD) {
        curr.pid
    } else {
        tid_num
    };
    let exit_signal = if flags.contains(CloneFlags::CLONE_THREAD) {
        SIGNONE
    } else {
        let sig = (flags & CloneFlags::CSIGNAL).bits() as usize;
        if !sigvalid(sig) {
            return Err(Errno::EINVAL);
        }
        sig
    };
    let new_task = Arc::new(Task {
        tid,
        pid,
        exit_signal,
        trapframe_tracker: Some(trapframe_tracker),
        state: SpinLock::new(TaskState::RUNNABLE),
        parent,
        children: SpinLock::new(LinkedList::new()),
        inner: SyncUnsafeCell::new(TaskInner {
            name: curr.inner().name.clone() + " (CLONED)",
            exit_code: 0,
            context: TaskContext::new(user_trap_return as usize, kstack_base),
            kstack,
            mm,
            files,
            set_child_tid,
            clear_child_tid,
            sig_pending: SigPending::new(),
            sig_blocked: SigSet::new(),
        }),
        fs_info,
        sig_actions,
    });

    // Set tid in parent address space
    if flags.contains(CloneFlags::CLONE_PARENT_SETTID) {
        let ptid = curr.mm().alloc_frame(ptid)?.start_address() + ptid.page_offset();
        unsafe { *(ptid.get_mut() as *mut i32) = tid_num as i32 };
    }

    // Set tid in child address space (COW)
    if flags.intersects(CloneFlags::CLONE_CHILD_SETTID | CloneFlags::CLONE_CHILD_CLEARTID) {
        let ctid = new_task.mm().alloc_frame(ctid)?.start_address() + ctid.page_offset();
        unsafe {
            *(ctid.get_mut() as *mut i32) = if flags.contains(CloneFlags::CLONE_CHILD_SETTID) {
                tid_num as i32
            } else {
                0
            }
        };
    }

    TASK_MANAGER.lock().add(new_task.clone());
    let p = new_task.parent.lock();
    if let Some(parent) = p.as_ref() {
        if let Some(parent) = parent.upgrade() {
            parent.children.lock().push_back(new_task.clone());
        }
    }
    Ok(tid_num)

}

/// A helper for [`syscall_interface::SyscallProc::execve`]
pub fn do_exec(dir: String, elf_data: &[u8], args: Vec<String>) -> SyscallResult {
    let curr = cpu().curr.as_ref().unwrap();
    log::trace!("EXEC {:?} ", &curr);
    log::trace!("EXEC {:?} DIR [{}] {:?}", &curr, &dir, &args);
    let args_len = args.len();
    // memory mappings are not preserved
    let mut mm = MM::new()?;
    let vsp = loader::from_elf(elf_data, &mut mm, args)?;

    // re-initialize kernel stack
    let kstack = KernelStack::new()?;
    let kstack_base = kstack.base();
    curr.inner().kstack = kstack;

    // re-initialize trapframe
    let trapframe = curr.trapframe();
    *trapframe = TrapFrame::new(
        KERNEL_SPACE.lock().page_table.satp(),
        kstack_base,
        user_trap_handler as usize,
        mm.entry.value(),
        vsp.into(),
    );
    trapframe.set_a1(vsp.into());
    mm.page_table
        .map(
            Page::from(VirtAddr::from(trapframe_base(curr.tid.0))),
            Frame::from(curr.trapframe_tracker.as_ref().unwrap().0.start_address()),
            PTEFlags::READABLE | PTEFlags::WRITABLE | PTEFlags::VALID,
        )
        .map_err(|_| KernelError::PageTableInvalid)?;
    log::trace!("{:?}", mm);

    curr.inner().mm = Arc::new(SpinLock::new(mm));
    curr.inner().context = TaskContext::new(user_trap_return as usize, kstack_base);
    Ok(args_len)
}

