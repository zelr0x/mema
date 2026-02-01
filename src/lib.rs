#![no_std]

#![feature(const_option_ops)]

pub mod direct_os;
pub mod bump;

pub use direct_os::DirectOsAllocator;

mod os;
mod util;

