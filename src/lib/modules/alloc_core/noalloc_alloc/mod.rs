use core::sync::atomic::AtomicUsize;

/// noalloc_alloc uses locked buckets to allocate memory without using the heap.

pub(self) static HEAP_START: AtomicUsize = AtomicUsize::new(0);
pub(self) const MANAGED_MAGIC: u32 = 0x6d656f77; // "meow" :3
// pub(self) const MAX_HEAP_SIZE: usize = 0x10000000; // 256MB
pub(self) const MAX_HEAP_SIZE: usize = 0x00200000; // 2MB
pub mod allocator;
pub(self) mod region;
pub(self) mod buckets;
pub(self) mod size;
pub(self) mod bucket_mgr;
pub(self) mod util;
pub(self) mod definition;

pub use allocator::BucketedAllocator;