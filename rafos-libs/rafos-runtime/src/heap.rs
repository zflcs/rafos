use core::alloc::{GlobalAlloc, Layout};

// Kernel must support thess function.
extern "C" {
    fn alloc(size: usize, align: usize) -> *mut u8;
    fn dealloc(ptr: *mut u8, size: usize, align: usize);
}

///
#[global_allocator]
static GLOBAL: Global = Global;

struct Global;
unsafe impl GlobalAlloc for Global {
    #[inline]
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let size = layout.size();
        let align = layout.align();
        alloc(size, align)
    }

    #[inline]
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let size = layout.size();
        let align = layout.align();
        dealloc(ptr, size, align);
    }
}

#[alloc_error_handler]
pub fn handle_alloc_error(layout: core::alloc::Layout) -> ! {
    panic!("Heap allocation error, layout = {:?}", layout);
}
