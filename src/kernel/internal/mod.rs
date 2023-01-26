pub(super) mod tables;
pub fn init() {
    tables::init();
}

pub(self) const DOUBLE_FAULT_IST_INDEX: u16 = 0;
pub const HEAP_START: usize = 0x_4444_4444_0000;
#[cfg(feature = "bump_allocator")] // slower allocator can't do as much as the faster one because we'd take longer to start up.
pub const HEAP_SIZE: usize = 512 * 1024;// * 1024;
#[cfg(not(feature = "bump_allocator"))] // slower allocator can't do as much as the faster one because we'd take longer to start up.
pub const HEAP_SIZE: usize = 1024 * 1024 * 1024; // 1 GiB, asynchronously allocated
#[cfg(feature = "bump_allocator")]
pub const HEAP_SIZE_AS_DEBUG_STR: &str = "512 KiB";
#[cfg(not(feature = "bump_allocator"))] // slower allocator can't do as much as the faster one because we'd take longer to start up.
pub const HEAP_SIZE_AS_DEBUG_STR: &str = "1 GiB";

