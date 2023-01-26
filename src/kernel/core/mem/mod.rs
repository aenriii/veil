mod alloc;
mod frame_allocator;
mod heap;
pub mod util;
use core::{ptr};

use bootloader::BootInfo;
use spin::Mutex;
use x86_64::{structures::paging::{Size4KiB, OffsetPageTable}, VirtAddr};

use crate::{kernel::internal::{HEAP_START, HEAP_SIZE, HEAP_SIZE_AS_DEBUG_STR}, log_text_mode, log, prealloc_log_vga};


pub(crate) use self::alloc::Allocator;
pub(crate) use frame_allocator::BootInfoFrameAllocator;

pub static PHYSICAL_OFFSET: Mutex<Option<u64>> = Mutex::new(None);
#[cfg(not(feature = "bump_allocator"))]
lazy_static::lazy_static! {
    pub(crate) static ref FrameAllocator: Mutex<BootInfoFrameAllocator<Size4KiB>> = Mutex::new(BootInfoFrameAllocator::new());
}
pub fn init(bi: &'static BootInfo) {
    *PHYSICAL_OFFSET.lock() = Some(bi.physical_memory_offset);
    #[cfg(feature = "bucket_allocator")] unsafe {
        Allocator.lock().init(bi)
     }
}