use core::alloc::{Layout};

use alloc::vec::Vec;
use bootloader::BootInfo;
use x86_64::{structures::paging::{Size4KiB, OffsetPageTable, page::PageRangeInclusive}, VirtAddr};
use x86_64::structures::paging::Page;

use crate::{kernel::core::mem::{BootInfoFrameAllocator, Allocator}, modules::{alloc_core::libregions::{Region, RegionIter}, async_core::{Executor, stable::StableExecutor}}, lib::veil_std::{no_alloc::BoundedIter, desync::{Executor, Task}, util::Locked}};

use super::AllocatorSettings;

static REGION_TABLE_ACCESS: spin::Mutex<()> = spin::Mutex::new(());
pub struct DynamicRegionAllocator<'a> {
    // settings: AllocatorSettings,

    heap_start: usize,
    heap_size: usize,

    current_heap_alloc: usize, // heap_start + current_heap_alloc = next preheap alloc location

    page_tables: Option<OffsetPageTable<'a>>,
    frame_allocator: Option<BootInfoFrameAllocator<Size4KiB>>,

    alloc_width: usize,
    region_tables: Option<Locked<Vec<Region>>>,

    emergency_region: Option<Region> // we use this to allocate the region_tables vec, and other "otherwise out of memory" allocations
}
impl <'a> DynamicRegionAllocator<'a> {
    pub const fn new(
        // settings: AllocatorSettings
    ) -> DynamicRegionAllocator<'a> {
        DynamicRegionAllocator {
            // settings,
            heap_start: 0,
            heap_size: 0,
            current_heap_alloc: 0,
            page_tables: None,
            frame_allocator: None,
            alloc_width: 4 * 1024,
            region_tables: None,
            emergency_region: None 
        }
    }
    pub unsafe fn init(&mut self, heap_start: usize, heap_size: usize, boot_info: &'static BootInfo) {
        self.heap_size = heap_size;
        self.heap_start = heap_start;

        self.page_tables = Some(OffsetPageTable::new(
            {
                {
                    use x86_64::registers::control::Cr3;

                    let (level_4_table_frame, _) = Cr3::read();

                    let phys = level_4_table_frame.start_address();
                    let virt = VirtAddr::new(boot_info.physical_memory_offset) + phys.as_u64();
                    let page_table_ptr: *mut x86_64::structures::paging::PageTable = virt.as_mut_ptr();

                    &mut *page_table_ptr // unsafe
                }},VirtAddr::new(boot_info.physical_memory_offset)));
        self.frame_allocator = Some(BootInfoFrameAllocator::new());
        self.frame_allocator.as_mut().unwrap().init(&boot_info.memory_map);

        self.emergency_region = Some(self.preheap_alloc(self.alloc_width));
        self.region_tables = Some(Locked::new(Vec::new())); // because of the emergency region, this should be able to allocate.

        let next_region = self.preheap_alloc(self.alloc_width);
        self.region_tables.as_ref().unwrap().lock().push(next_region);
    }

    /// pages {x} bytes of memory and allows it to be used by the allocator
    fn preheap_alloc(&mut self, bytes: usize) -> Region {
        let start = self.heap_start + self.current_heap_alloc;
        self.current_heap_alloc += bytes;
        let end = self.heap_start + self.current_heap_alloc;
        
        let start = VirtAddr::new(start as u64);
        let end = VirtAddr::new(end as u64);

        fn page_range_from(first: VirtAddr, second: VirtAddr) -> PageRangeInclusive {
            let first_page = Page::containing_address(first);
            let last_page = Page::containing_address(second);
            Page::range_inclusive(first_page, last_page)
        }

        fn map_page(page: Page, frame_allocator: &mut BootInfoFrameAllocator<Size4KiB>, page_tables: &mut OffsetPageTable) {
            use x86_64::structures::paging::{PageTableFlags, Mapper, FrameAllocator};
            let frame = frame_allocator.allocate_frame().expect("no more frames");
            let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;
            let map_to_result = unsafe {
                page_tables.map_to(page, frame, flags, frame_allocator)
            };
            map_to_result.expect("map_to failed").flush();
        }

        let page_range = page_range_from(start, end);
        for page in page_range {
            map_page(page, self.frame_allocator.as_mut().unwrap(), self.page_tables.as_mut().unwrap());
        }
        Region::new(start, end)
    }
    fn alloc_from_region(&mut self, layout: Layout, region: Region) -> (Option<BoundedIter<Region, 2>>, Option<*mut u8>) {
        let size = layout.size();
        let align = layout.align();
        let mut region = region;
        if region.can_fit_aligned(size, align) {
            let mut ret = region.chunk_aligned(size, align);
            return (ret.0, ret.1.map(|x| x.start_addr().as_u64() as *mut u8))
        }

        (None, None)
    }
    
    pub fn internal_alloc(&mut self, layout: Layout) -> *mut u8 {

        
        if let None = self.region_tables {
            let (regions, ptr) = self.alloc_from_region(layout, self.emergency_region.unwrap());
            if let Some(regions) = regions {
                let task = Task::new(async move {unsafe {
                    // TODO
                }});
                unsafe {Executor.borrow().tasks.write().insert(task.id, task);}
            }
            if let Some(ptr) = ptr {
                return ptr
            }
            
        } else if let Some(tables) = &self.region_tables {
            if let Some(ptr) = tables.allocate_aligned(layout.size(), layout.align()) {
                return ptr
            } else {
                #[cfg(feature = "serial_stdout")]
                crate::serial_println!("WARNING! returning nullptr!")
            }
        }

        0 as *mut u8 // !! if you ever see this, something went wrong
    }

    pub fn internal_dealloc(&mut self, ptr: *mut u8, size: usize) {
        if let Some(tables) = &self.region_tables {
            tables.deallocate(ptr, size)
        }
    }



}
