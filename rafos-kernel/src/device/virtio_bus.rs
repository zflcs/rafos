use crate::mm::KERNEL_SPACE;

use mmrv::*;
use alloc::vec::Vec;
use spin::{Lazy, Mutex};
use virtio_drivers::{Hal, PhysAddr, VirtAddr};

pub static QUEUE_FRAMES: Lazy<Mutex<Vec<AllocatedFrameRange>>> = Lazy::new(|| Mutex::new(Vec::new()));

pub struct VirtioHal;

impl Hal for VirtioHal {
    fn dma_alloc(pages: usize) -> PhysAddr {
        let trakcers = AllocatedFrameRange::new(pages, true).unwrap();
        let pa = trakcers.start.start_address();
        QUEUE_FRAMES.lock().push(trakcers);
        pa.value()
    }

    fn dma_dealloc(paddr: PhysAddr, pages: usize) -> i32 {
        let mut ranges = QUEUE_FRAMES.lock();
        let (idx, _) = ranges
            .iter()
            .enumerate()
            .find(|(_, frames)| 
                frames.contains_address(paddr.into())
            ).unwrap();
        let frames = ranges.remove(idx);
        frame_dealloc(frames.start.number(), pages);
        0
    }

    fn phys_to_virt(paddr: PhysAddr) -> VirtAddr {
        paddr
    }

    fn virt_to_phys(vaddr: VirtAddr) -> PhysAddr {
        KERNEL_SPACE.lock().translate(vaddr.into()).unwrap().value()
    }
}
