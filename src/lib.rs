extern crate fs2;
extern crate libc;
extern crate mmap;

#[cfg(target_os = "linux")]
mod linux;

#[cfg(target_os = "linux")]
pub use linux::*;

pub use mmap::MemoryMap;
