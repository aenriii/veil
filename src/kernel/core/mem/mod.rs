mod alloc;
mod frame_allocator;
mod heap;
use core::{ptr};

use bootloader::BootInfo;
use spin::Mutex;
use x86_64::{structures::paging::{Size4KiB, OffsetPageTable}, VirtAddr};

use crate::{kernel::internal::{HEAP_START, HEAP_SIZE, HEAP_SIZE_AS_DEBUG_STR}, log_text_mode, log, prealloc_log_vga};
#[cfg(feature = "bump_allocator")]

pub(super) use self::alloc::Allocator;
#[cfg(feature = "mithril_allocator")] // we need this available if we're using the mithril allocator
pub(crate) use self::alloc::Allocator;
#[cfg(feature = "bump_allocator")]
pub(super) use frame_allocator::BootInfoFrameAllocator;
#[cfg(feature = "mithril_allocator")] // we need this available if we're using the mithril allocator
pub(crate) use frame_allocator::BootInfoFrameAllocator;
#[cfg(feature = "bump_allocator")]
lazy_static::lazy_static! {
    pub(super) static ref FrameAllocator: Mutex<BootInfoFrameAllocator<Size4KiB>> = Mutex::new(BootInfoFrameAllocator::new());
}
#[cfg(feature = "mithril_allocator")]
lazy_static::lazy_static! {
    pub(crate) static ref FrameAllocator: Mutex<BootInfoFrameAllocator<Size4KiB>> = Mutex::new(BootInfoFrameAllocator::new());
}

#[cfg(feature = "bump_allocator")]
pub(super) static PageTable: Mutex<Option<OffsetPageTable>> = Mutex::new(None);
#[cfg(feature = "mithril_allocator")]
pub(crate) static PageTable: Mutex<Option<OffsetPageTable>> = Mutex::new(None);

pub fn init(bi: &'static BootInfo) {
    unsafe {
        let mut opt = OffsetPageTable::new(
            {
                {
                    use x86_64::registers::control::Cr3;

                    let (level_4_table_frame, _) = Cr3::read();

                    let phys = level_4_table_frame.start_address();
                    let virt = VirtAddr::new(bi.physical_memory_offset) + phys.as_u64();
                    let page_table_ptr: *mut x86_64::structures::paging::PageTable = virt.as_mut_ptr();

                    &mut *page_table_ptr // unsafe
                }},VirtAddr::new(bi.physical_memory_offset));
        {*PageTable.lock() = Some(opt);}
        FrameAllocator.lock().init(&bi.memory_map);
        // prealloc_log_vga!("[mem::init] Allocating {} of HEAP, from {:#} to {:#}", HEAP_SIZE_AS_DEBUG_STR, HEAP_START, HEAP_START + HEAP_SIZE);
        // Allocator.lock().init(HEAP_START, HEAP_SIZE);
        prealloc_log_vga!("Heaping...");
        match heap::init() {
            Ok(_) => log!("Heap initialized!"),
            Err(e) => panic!("Heap initialization failed: {:#?}", e)
        }
    }
}