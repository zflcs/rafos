use config::KERNEL_HEAP_SIZE;
use buddy_system_allocator::LockedHeap;

#[alloc_error_handler]
pub fn handle_alloc_error(layout: core::alloc::Layout) -> ! {
    panic!("Heap allocation error, layout = {:?}", layout)
}

#[no_mangle]
#[link_section = ".data.heap"]
#[global_allocator]
static HEAP: LockedHeap<32> = LockedHeap::new();


#[no_mangle]
#[link_section = ".bss.memory"]
static mut HEAP_SPACE: [u8; KERNEL_HEAP_SIZE] = [0u8; KERNEL_HEAP_SIZE];

/// 初始化全局分配器和内核堆分配器。
pub fn init_heap() {
    unsafe {
        HEAP.lock().init(HEAP_SPACE.as_ptr() as usize, KERNEL_HEAP_SIZE);
    }
}

