#![allow(dead_code)]  // It's fine for the time being.

pub(crate) mod alignments;

pub(crate) use alignments::*;

#[inline(always)]
pub(crate) const fn align_up(addr: usize, align: usize) -> usize {
    debug_assert!(align.is_power_of_two());
    (addr + align - 1) & !(align - 1)
}

#[inline(always)]
pub(crate) fn align_up_ptr(ptr: *mut u8, align: usize) -> *mut u8 {
    align_up(ptr as usize, align) as *mut u8
}

#[inline(always)]
pub(crate) fn checked_align_up(addr: usize, align: usize) -> Option<usize> {
    debug_assert!(align.is_power_of_two());
    addr.checked_add(align - 1)
        .map(|x| x & !(align - 1))
}

#[inline(always)]
pub(crate) fn checked_align_up_ptr(ptr: *mut u8, align: usize) -> *mut u8 {
    checked_align_up(ptr as usize, align)
        .map_or(core::ptr::null_mut(), |x| x as *mut u8)
}
