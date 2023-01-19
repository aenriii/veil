use crate::{print, println};

use x86_64::{structures::paging::{Mapper, Size4KiB, FrameAllocator, mapper::MapToError, Page, PageTableFlags}, VirtAddr};

use crate::{serial_println, screen::vga};

pub const HEAP_START: usize = 0x_4444_4444_0000;
pub const HEAP_SIZE: usize = 10 * 1024 * 1024; // 10 Mb


pub fn init(
    mapper: &mut impl Mapper<Size4KiB>,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
) -> Result<(), MapToError<Size4KiB>> {
    let page_range = {
        let heap_start = VirtAddr::new(HEAP_START as u64);
        let heap_end = heap_start + HEAP_SIZE - 1u64;
        let heap_start_page = Page::containing_address(heap_start);
        let heap_end_page = Page::containing_address(heap_end);
        serial_println!("[heap::init] heap_start_page: {:#x}", heap_start_page.start_address().as_u64());
        Page::range_inclusive(heap_start_page, heap_end_page)
    };
    serial_println!("[heap::init] page opened");
    serial_println!("[heap::init] mapping pages (count: {})...", page_range.count());
    let mut count_pages_mapped = 0;
    let count_pages = page_range.count();
    print!("[heap::init] Mapping pages: {}/{}", count_pages_mapped, count_pages);
    for page in page_range {
        let frame = frame_allocator
            .allocate_frame()
            .ok_or(MapToError::FrameAllocationFailed)?;
        let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;
        unsafe {
            mapper.map_to(page, frame, flags, frame_allocator)?.flush()
        };
        count_pages_mapped += 1;
        vga::goto_line_start();
        print!("[heap::init] Mapping pages: {}/{}", count_pages_mapped, count_pages);
    }
    println!();
    serial_println!("[heap::init] pages mapped");

    unsafe {
        serial_println!("heap set at {:#x} - {:#x}", HEAP_START, HEAP_START + HEAP_SIZE);
        serial_println!("heap size: {} bytes", HEAP_SIZE);
        serial_println!("Initializing global allocator...");
        super::alloc::Allocator.lock().init(HEAP_START, HEAP_SIZE);
    }
    Ok(())
}