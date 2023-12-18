#![no_std]

extern crate alloc;

use alloc::vec::Vec;
use core::option::Option::Some;

/// Allocate identifications using different algorithms
pub trait IDAllocator {
    fn alloc(&mut self) -> usize;
    fn dealloc(&mut self, id: usize);
}

pub struct RecycleAllocator {
    current: usize,
    recycled: Vec<usize>,
}

impl RecycleAllocator {
    pub fn new(current: usize) -> Self {
        Self {
            current,
            recycled: Vec::new(),
        }
    }
}

impl IDAllocator for RecycleAllocator {
    fn alloc(&mut self) -> usize {
        if let Some(id) = self.recycled.pop() {
            id
        } else {
            self.current += 1;
            assert_ne!(self.current, usize::MAX);
            self.current - 1
        }
    }

    fn dealloc(&mut self, id: usize) {
        self.recycled.push(id);
    }
}

