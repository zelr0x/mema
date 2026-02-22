use core::ops::{Deref, DerefMut};

/// Align64 provides false sharing protection with 64 bytes of alignment.
#[repr(align(64))]
pub(crate) struct Align64<T>(pub T);

impl<T> Align64<T> {
    pub const fn new(inner: T) -> Self {
        Align64(inner)
    }
}

impl<T> Deref for Align64<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Align64<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// Align128 defines false sharing protection policy taking into account
/// that on Intel CPUs starting with Sandy Bridge, the adjacent line prefetcher
/// speculatively loads the neighboring cache line when one is accessed.
/// Because both lines may then participate in coherence traffic, writes to
/// one atomic can effectively cause invalidations of its neighbor as well.
#[repr(align(128))]
pub(crate) struct Align128<T>(pub T);

impl<T> Align128<T> {
    pub const fn new(inner: T) -> Self {
        Align128(inner)
    }
}

impl<T> Deref for Align128<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Align128<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
