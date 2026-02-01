#![allow(dead_code)]

use core::ffi::c_void;

// FFI for memoryapi.h

// flAllocationType

/// Allocate memory charges (from memory and paging file).
/// Allocates only on access. Memory is guaranteed to be zeroed on access.
/// Can't commit unreserved (must be reserved before or at the same time).
/// Can commit twice.
pub const MEM_COMMIT: u32      = 0x00001000;

/// Reserve a range without allocating any actual physical storage.
/// Other memory allocation functions (malloc, LocalAlloc) can't use reserved
/// memory until it's released.
pub const MEM_RESERVE: u32     = 0x00002000;

/// Memory is no longer of interest - pages should not be read from or written to
/// a paging file, but it will be used later, so don't decommit.
/// Can't be used with other flags. Ignores flProtect, but it still must be set
/// to a valid value lile PAGE_NOACCESS.
pub const MEM_RESET: u32       = 0x00080000;

/// Undo a reset - only call on memory range to which MEM_RESET was successfully
/// applied earlier. On success all the data remains intact. On fail it is
/// at least partially zeroed.
/// Can't be used with other flags. Ignores flProtect, but it still must be set
/// to a valid value lile PAGE_NOACCESS.
/// Not supported until Windows 8 / Windows Server 2012.
pub const MEM_RESET_UNDO: u32  = 0x01000000;

// TODO: document flags below.

// flAllocationType additional
pub const MEM_LARGE_PAGES: u32 = 0x20000000;
pub const MEM_PHYSICAL: u32    = 0x00400000;
pub const MEM_TOP_DOWN: u32    = 0x00100000;
pub const MEM_WRITE_WATCH: u32 = 0x00200000;

// flProtect (memory protection constants)
pub const PAGE_EXECUTE: u32           = 0x10;
pub const PAGE_EXECUTE_READ: u32      = 0x20;
pub const PAGE_EXECUTE_READWRITE: u32 = 0x40;
pub const PAGE_EXECUTE_WRITECOPY: u32 = 0x80;
pub const PAGE_NOACCESS: u32          = 0x01;
pub const PAGE_READONLY: u32          = 0x02;
pub const PAGE_READWRITE: u32         = 0x04;
pub const PAGE_WRITECOPY: u32         = 0x08;
pub const PAGE_TARGETS_INVALID: u32   = 0x40000000;
pub const PAGE_TARGETS_NO_UPDATE: u32 = 0x40000000;

// flProtect additional flags
pub const PAGE_GUARD: u32             = 0x100;
pub const PAGE_NOCACHE: u32           = 0x200;
pub const PAGE_WRITECOMBINE: u32      = 0x400;

// dwFreeType
pub const MEM_DECOMMIT: u32 = 0x00004000;
pub const MEM_RELEASE:  u32 = 0x00008000;

// dwFreeType additional flags for MEM_RELEASE
pub const MEM_COALESCE_PLACEHOLDERS: u32 = 0x00000001;
pub const MEM_PRESERVE_PLACEHOLDER:  u32 = 0x00000002;

#[link(name = "kernel32")]
extern "system" {
    pub fn VirtualAlloc(
        lpAddress: *mut c_void,
        dwSize: usize,
        flAllocationType: u32,
        flProtect: u32,
    ) -> *mut c_void;

    pub fn VirtualFree(
        lpAddress: *mut c_void,
        dwSize: usize,
        dwFreeType: u32,
    ) -> i32;

    // TODO: add HeapAlloc?
}

pub(crate) unsafe fn reserve(size: usize) -> *mut u8 {
    VirtualAlloc(
        core::ptr::null_mut(), size,
        MEM_COMMIT | MEM_RESERVE,
        PAGE_READWRITE) as *mut u8
}

/// If MEM_RELEASE is used, dwSize must be 0 -  the entire region that
/// is reserved in the initial allocation call to VirtualAlloc will be freed.
pub(crate) unsafe fn release(ptr: *mut u8, _size: usize) {
    VirtualFree(ptr as *mut c_void, 0, MEM_RELEASE);
}

