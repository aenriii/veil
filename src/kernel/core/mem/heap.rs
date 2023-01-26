use core::ops::DerefMut;

use lazy_static::__Deref;
use x86_64::{
    structures::paging::{
        mapper::MapToError, FrameAllocator as FA_tr, Mapper, Page, PageTableFlags, Size4KiB,
    },
    VirtAddr,
};

use crate::{
    kernel::{
        internal::{HEAP_SIZE, HEAP_START},
    },
    print, println, serial_println,
};

pub fn init() -> Result<(), MapToError<Size4KiB>> {
    #[cfg(feature = "_prepage_heap")]
    {
        use crate::kernel::core::mem::{Allocator, FrameAllocator, PageTable};
        let mut mapper = PageTable.lock();
        let mut frame_allocator = FrameAllocator.lock();
        let page_range = {
            let heap_start = VirtAddr::new(HEAP_START as u64);
            let heap_end = heap_start + HEAP_SIZE - 1u64;
            let heap_start_page = Page::containing_address(heap_start);
            let heap_end_page = Page::containing_address(heap_end);
            serial_println!(
                "[heap::init] heap_start_page: {:#x}",
                heap_start_page.start_address().as_u64()
            );
            Page::range_inclusive(heap_start_page, heap_end_page)
        };
        serial_println!("[heap::init] page opened");
        serial_println!(
            "[heap::init] mapping pages (count: {})...",
            page_range.count()
        );
        let mut count_pages_mapped = 0;
        let count_pages = page_range.count();
        // print!("[heap::init] Mapping pages: {}/{}", count_pages_mapped, count_pages);
        for page in page_range {
            let frame = frame_allocator
                .allocate_frame()
                .ok_or(MapToError::FrameAllocationFailed)?;
            let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;
            unsafe {
                // serial_println!("mapper IN");
                mapper
                    .as_mut()
                    .unwrap()
                    .map_to(page, frame, flags, frame_allocator.deref_mut())?
                    .flush();
                // serial_println!("mapper OUT");
            };
            count_pages_mapped += 1;
            // #[cfg(feature = "vga_text_mode")]
            // VgaTextWriter.lock().restart_line();
            // print!("[heap::init] Mapping pages: {}/{}", count_pages_mapped, count_pages);
        }
        // println!();
        serial_println!("[heap::init] pages mapped");

        unsafe {
            serial_println!(
                "heap set at {:#x} - {:#x}",
                HEAP_START,
                HEAP_START + HEAP_SIZE
            );
            serial_println!("heap size: {} bytes", HEAP_SIZE);
            serial_println!("Initializing global allocator...");
            super::alloc::Allocator.lock().init(HEAP_START, HEAP_SIZE);
        }
        return Ok(())
    }
    #[cfg(not(feature = "_prepage_heap"))]  #[cfg(not(feature = "dra_allocator"))] #[cfg(not(feature = "noalloc_allocator"))] unsafe {
        serial_println!("[heap::init] prepage_heap feature not enabled");
        // assume heap takes in more parameters
        super::alloc::Allocator.lock().init(HEAP_START, HEAP_SIZE);
        Ok(())
    }

    Ok(())
}
