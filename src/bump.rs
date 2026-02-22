use core::alloc::{GlobalAlloc, Layout};
use core::cell::UnsafeCell;
use core::ptr::{NonNull, null_mut};
use core::sync::atomic::{AtomicPtr, AtomicU8, Ordering};
use crate::util::align::{Align64, align_up_ptr};
#[cfg(any(unix, windows))]
use crate::os;

// Static heap is needed for allocations ran before main (hence, before init()).
// TODO: Make it configurable.
const STATIC_HEAP_SIZE: usize = 64 * 1024;
static mut STATIC_HEAP: [u8; STATIC_HEAP_SIZE] = [0; STATIC_HEAP_SIZE];

const UNINIT: u8 = 0;
const INIT_IN_PROGRESS: u8 = 1;
const INIT_DONE: u8 = 2;

pub struct BumpAllocator {
    start:   UnsafeCell<*mut u8>,
    end:     UnsafeCell<*mut u8>,
    current: Align64<AtomicPtr<u8>>,
    init_flag: Align64<AtomicU8>,
}

impl BumpAllocator {

    pub const fn new() -> Self {
        Self {
            start: UnsafeCell::new(null_mut()),
            end: UnsafeCell::new(null_mut()),
            current: Align64::new(AtomicPtr::new(null_mut())),
            init_flag: Align64::new(AtomicU8::new(UNINIT)),
        }
    }

    // TODO: guard against repeated calls.
    /// Must be called once.
    pub unsafe fn init(&self, start: Option<*mut u8>, size: usize) {
        let ptr = match start {
            Some(ptr) => ptr,
            None => os::reserve(size),
        };
        *self.start.get() = ptr;
        *self.end.get() = ptr.add(size);
        self.current.store(ptr, Ordering::Relaxed);
        self.init_flag.store(INIT_DONE, Ordering::Release);
    }

    #[inline(always)]
    unsafe fn ensure_init(&self) {
        //if !(*self.start.get()).is_null() {
        if self.init_flag.load(Ordering::Acquire) == INIT_DONE {
           return;
        }
        // try to become the initializing thread
        if self.init_flag
            .compare_exchange(UNINIT, INIT_IN_PROGRESS,
                              Ordering::AcqRel, Ordering::Relaxed)
            .is_ok()
        {
            #[allow(static_mut_refs)]
            let heap: *mut u8 = STATIC_HEAP.as_mut_ptr();
            *self.start.get() = heap;
            *self.end.get() = heap.add(STATIC_HEAP_SIZE);
            self.current.store(heap, Ordering::Relaxed);
            self.init_flag.store(INIT_DONE, Ordering::Release);
        } else {
            // Other threads spin until init is done
            while self.init_flag.load(Ordering::Acquire) != INIT_DONE {
                core::hint::spin_loop();
            }
        }
    }
}

unsafe impl Sync for BumpAllocator {}

unsafe impl GlobalAlloc for BumpAllocator {

    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let size = layout.size();
        if size == 0 {
            return NonNull::<u8>::dangling().as_ptr();
        }
        self.ensure_init();

        let align = layout.align();
        let end_addr = (*self.end.get()) as usize;
        let mut curr = self.current.load(Ordering::Relaxed);
        loop {
            let aligned = align_up_ptr(curr, align);
            let aligned_addr = aligned as usize;
            let next = aligned_addr + size;
            if next > end_addr {
                return null_mut()
            }
            let next = next as *mut u8;
            match self.current.compare_exchange(curr, next,
                                                Ordering::AcqRel,
                                                Ordering::Relaxed) {
                Ok(_) => return aligned,
                Err(actual) => curr = actual,
            }
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let size = layout.size();
        let align = layout.align();
        let aligned_start = align_up_ptr(ptr, align);
        let end = aligned_start.add(size);
        let curr = self.current.load(Ordering::Relaxed);
        if curr == end {
            let _ = self.current.compare_exchange(
                curr,
                ptr,
                Ordering::Relaxed,
                Ordering::Relaxed,
            );
        }
        // Well, it's an allocator, not a deallocator.
    }
}

