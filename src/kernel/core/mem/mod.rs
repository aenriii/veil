mod alloc;
mod frame_allocator;
mod heap;
use core::{ptr};

use bootloader::BootInfo;
use spin::Mutex;
use x86_64::{structures::paging::{Size4KiB, OffsetPageTable}, VirtAddr};

use crate::{kernel::internal::{HEAP_START, HEAP_SIZE, HEAP_SIZE_AS_DEBUG_STR}, log_text_mode, log, prealloc_log_vga};

pub(crate) use self::alloc::Allocator;
pub(crate) use frame_allocator::BootInfoFrameAllocator;
#[cfg(not(feature = "bump_allocator"))]
lazy_static::lazy_static! {
    pub(crate) static ref FrameAllocator: Mutex<BootInfoFrameAllocator<Size4KiB>> = Mutex::new(BootInfoFrameAllocator::new());
}
pub fn init(bi: &'static BootInfo) {
    #[cfg(feature = "bucket_allocator")] unsafe {
        Allocator.lock().init(bi)
     }
}