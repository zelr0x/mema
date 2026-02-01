use mema::direct_os::DirectOsAllocator;

#[global_allocator]
static A: DirectOsAllocator = DirectOsAllocator::new();

mod common;

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
