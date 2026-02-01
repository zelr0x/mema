use ctor::ctor;

use mema::bump::BumpAllocator;

#[global_allocator]
static A: BumpAllocator = BumpAllocator::new();

#[ctor]
unsafe fn init() {
    A.init(None, 8 * 1024 * 1024);
}

mod common;

// TODO: rewrite to data-based.

#[test]
fn test_box_alloc() {
    common::test_box_alloc();
}

#[test]
fn test_vec_alloc() {
    common::test_vec_alloc();
}

#[test]
fn test_alignment() {
    common::test_alignment();
}

#[test]
fn test_alignment_matrix() {
    common::test_alignment_matrix();
}

#[test]
fn test_zst_box() {
    common::test_zst_box();
}

#[test]
fn test_zst_alignment() {
    common::test_zst_alignment();
}

#[test]
fn test_zst_vec_capacity() {
    common::test_zst_vec_capacity();
}

#[test]
fn test_threadsafe_no_overlap_alloc_only() {
    common::test_threadsafe_no_overlap_alloc_only(&A);
}

#[test]
fn test_oom_is_graceful() {
    common::test_oom_is_graceful(&A);
}

// --- bump only ---
#[test]
fn test_zst_does_not_advance_bump() {
    common::test_zst_does_not_advance_bump(&A);
}

