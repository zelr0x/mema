use core::alloc::{GlobalAlloc, Layout};
use crate::os;

pub struct DirectOsAllocator {
}

impl DirectOsAllocator {

    pub const fn new() -> Self {
        DirectOsAllocator{}
    }
}

unsafe impl GlobalAlloc for DirectOsAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        os::reserve(layout.size())
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        os::release(ptr, layout.size());
    }
}

