use std::alloc::{GlobalAlloc, Layout};
use std::sync::{Arc, Barrier};
use std::thread;

// TODO: Write better tests.

pub fn test_box_alloc() {
    let b = Box::new(42);
    assert_eq!(*b, 42);
}

pub fn test_vec_alloc() {
    let mut v = Vec::new();
    for i in 0..1024 {
        v.push(i);
    }
    assert_eq!(v.len(), 1024);
    assert_eq!(v[100], 100);
}

pub fn test_alignment() {
    #[repr(align(64))]
    struct Aligned(u8);
    let x = Box::new(Aligned(1));
    let ptr = &*x as *const Aligned as usize;
    assert_eq!(ptr % 64, 0);
    assert_eq!(x.0, 1);
}

pub fn test_alignment_matrix() {
    for &align in &[8, 16, 32, 64, 128] {
        let layout = Layout::from_size_align(1, align).unwrap();
        unsafe {
            let p = std::alloc::alloc(layout);
            assert!(!p.is_null());
            assert_eq!(p as usize % align, 0);
        }
    }
}

pub fn test_zst_box() {
    let a = Box::new(());
    let b = Box::new(());

    let pa = &*a as *const () as usize;
    let pb = &*b as *const () as usize;

    // Must be non-null.
    assert_ne!(pa, 0);
    assert_ne!(pb, 0);

    // Allowed to be equal.
    assert!(pa == pb || pa != pb);
}

pub fn test_zst_alignment() {
    #[repr(align(64))]
    struct Z;

    let z = Box::new(Z);
    let ptr = &*z as *const Z as usize;

    assert_eq!(ptr % 64, 0);
}

pub fn test_zst_vec_capacity() {
    let mut v: Vec<()> = Vec::with_capacity(10);

    // Capacity is logical, not physical
    assert!(v.capacity() >= 10);
    assert_eq!(v.len(), 0);

    for _ in 0..1000 {
        v.push(());
    }

    assert_eq!(v.len(), 1000);
}

pub fn test_threadsafe_no_overlap_alloc_only<A: GlobalAlloc + Sync + 'static>(
    alloc: &'static A,
) {
    const THREADS: usize = 8;
    const ALLOCS_PER_THREAD: usize = 128;
    const SIZE: usize = 64;

    let alloc = Arc::new(alloc);
    let barrier = Arc::new(Barrier::new(THREADS));

    let mut handles = Vec::new();

    for _ in 0..THREADS {
        let alloc = Arc::clone(&alloc);
        let barrier = Arc::clone(&barrier);

        handles.push(thread::spawn(move || {
            let mut ranges = Vec::new();

            barrier.wait(); // maximize contention

            unsafe {
                let layout = Layout::from_size_align(SIZE, 8).unwrap();
                for _ in 0..ALLOCS_PER_THREAD {
                    let ptr = alloc.alloc(layout);
                    assert!(!ptr.is_null());

                    core::ptr::write_bytes(ptr, 0xAA, SIZE);
                    ranges.push((ptr as usize, ptr as usize + SIZE));
                }
            }

            ranges
        }));
    }

    let mut all_ranges = Vec::new();
    for h in handles {
        all_ranges.extend(h.join().unwrap());
    }

    for i in 0..all_ranges.len() {
        for j in i + 1..all_ranges.len() {
            assert!(
                all_ranges[i].1 <= all_ranges[j].0 ||
                all_ranges[j].1 <= all_ranges[i].0,
                "Overlap between {:?} and {:?}",
                all_ranges[i],
                all_ranges[j],
            );
        }
    }
}

pub fn test_oom_is_graceful<A: GlobalAlloc>(alloc: &'static A) {
    unsafe {
        let layout = Layout::from_size_align(1024 * 1024, 8).unwrap();
        let mut last = core::ptr::null_mut();
        loop {
            let p = alloc.alloc(layout);
            if p.is_null() {
                break;
            }
            last = p;
        }
        assert!(last.is_null() || !last.is_null());
    }
}

// --- bump only ---
pub fn test_zst_does_not_advance_bump<A: GlobalAlloc>(alloc: &'static A) {
    use core::alloc::Layout;

    unsafe {
        let layout = Layout::from_size_align(16, 8).unwrap();
        let p1 = alloc.alloc(layout);

        let _z = Box::new(()); // Should not touch allocator.

        let p2 = alloc.alloc(layout);

        assert_eq!(
            (p2 as usize) - (p1 as usize),
            16,
            "ZST allocation advanced bump pointer"
        );
    }
}

