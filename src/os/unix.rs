#![allow(dead_code)]

use core::ffi::{c_void, c_int, c_size_t};

// FFI for mman.h

// Impl might not support PROT_WRITE or PROT_EXEC without PROT_READ.
pub const PROT_NONE: c_int      = 0x0;  // Page cannot be accessed.
pub const PROT_READ: c_int      = 0x1;  // Page can be read.
pub const PROT_WRITE: c_int     = 0x2;  // Page can be written.
pub const PROT_EXEC: c_int      = 0x4;  // Page can be executed.

// Sharing types (must choose only one of these)
pub const MAP_SHARED: c_int     = 0x01; // Share changes.
pub const MAP_PRIVATE: c_int    = 0x02; // Changes are private.

pub const MAP_FIXED: c_int      = 0x10; // Interpret addr exactly.
pub const MAP_ANON: c_int       = 0x20; // BSD/macOS
pub const MAP_ANONYMOUS: c_int  = 0x20; // Linux

#[link(name = "c")]
extern "C" {
    pub fn mmap(
        addr: *mut c_void,
        length: c_size_t,
        prot: c_int,
        flags: c_int,
        fd: c_int,
        offset: isize,
    ) -> *mut c_void;

    pub fn munmap(
        addr: *mut c_void,
        length: c_size_t,
    ) -> c_int;

    pub fn brk(addr: *mut c_void) -> c_int;
    pub fn sbrk(increment: isize) -> *mut c_void;
}

pub(crate) unsafe fn reserve(size: usize) -> *mut u8 {
    // ANON makes this call ask for memory, not a file.
    // fd must be passed, so pass -1, same with offset (pass 0).
    mmap(core::ptr::null_mut(), size,
         PROT_READ | PROT_WRITE,
         MAP_PRIVATE | MAP_ANON,
         -1, 0) as *mut u8
}

pub(crate) unsafe fn release(ptr: *mut u8, size: usize) {
    // TODO: add some check in case c_size_t differs from usize?
    munmap(ptr as *mut c_void, size as c_size_t);
}

