use x86_64::{VirtAddr, structures::paging::{page::PageRangeInclusive, Page, OffsetPageTable, Size2MiB, Size4KiB, PageSize, Mapper}};

use crate::kernel::core::mem::BootInfoFrameAllocator;


pub fn page_range_from<S : PageSize>(first: VirtAddr, second: VirtAddr) -> PageRangeInclusive<S> {
    let first_page = Page::containing_address(first);
    let last_page = Page::containing_address(second);
    Page::<S>::range_inclusive(first_page, last_page)
}
pub fn map_page(page: Page, frame_allocator: &mut BootInfoFrameAllocator<Size4KiB>, page_tables: &mut OffsetPageTable) {
    use x86_64::structures::paging::{PageTableFlags, Mapper, FrameAllocator};
    let frame = frame_allocator.allocate_frame().expect("no more frames");
    let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;
    let map_to_result = unsafe {
        page_tables.map_to(page, frame, flags, frame_allocator)
        
    };
    map_to_result.expect("map_to failed").flush();
}