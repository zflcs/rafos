
#![no_std]
#![no_main]
#![feature(alloc_error_handler, lang_items)]
#![allow(internal_features)]

pub const HEAP_SIZE: usize = 0x80_000;
use buddy_system_allocator::LockedHeap;
use core::{
    alloc::{GlobalAlloc, Layout},
    ptr::NonNull,
};
use spin::Lazy;
core::arch::global_asm!(include_str!("info.asm"));



#[no_mangle]
pub unsafe extern "C" fn rafos_alloc(size: usize, align: usize) -> *mut u8 {
    HEAP.lock()
        .alloc(Layout::from_size_align(size, align).unwrap())
        .ok()
        .map_or(0 as *mut u8, |allocation| allocation.as_ptr())
}

#[no_mangle]
pub unsafe extern "C" fn rafos_dealloc(ptr: *mut u8, size: usize, align: usize) {
    HEAP.lock().dealloc(
        NonNull::new_unchecked(ptr), 
        Layout::from_size_align(size, align).unwrap()
    )
}


#[no_mangle]
#[link_section = ".data.heap"]
pub static mut HEAP: Lazy<LockedHeap<32>> = Lazy::new(|| {
    let heap = LockedHeap::new();
    unsafe {
        heap.lock().init(MEMORY.as_ptr() as usize, HEAP_SIZE);
    }
    heap
});


#[no_mangle]
#[link_section = ".bss.memory"]
static mut MEMORY: [u8; HEAP_SIZE] = [0u8; HEAP_SIZE];

struct Global;

#[global_allocator]
static GLOBAL: Global = Global;

unsafe impl GlobalAlloc for Global {
    #[inline]
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        HEAP.lock()
            .alloc(layout)
            .ok()
            .map_or(0 as *mut u8, |allocation| allocation.as_ptr())
    }

    #[inline]
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        HEAP.lock().dealloc(NonNull::new_unchecked(ptr), layout)
    }
}




pub mod lang_item {
    ///
    #[lang = "eh_personality"]
    #[no_mangle]
    pub fn rust_eh_personality() {}

    /// not_kernel panic
    #[panic_handler]
    fn panic(_info: &core::panic::PanicInfo) -> ! {
        unreachable!()
    }
}

#[alloc_error_handler]
pub fn handle_alloc_error(layout: core::alloc::Layout) -> ! {
    panic!("Heap allocation error, layout = {:?}", layout);
}
