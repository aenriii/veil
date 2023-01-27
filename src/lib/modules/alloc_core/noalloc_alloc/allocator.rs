use core::alloc::GlobalAlloc;

use bootloader::BootInfo;
use lock_api::RawMutex;
use x86_64::{structures::paging::{Size2MiB, FrameAllocator, Size4KiB, OffsetPageTable}, VirtAddr};

use crate::{lib::veil_std::util::Locked, kernel::{core::mem::BootInfoFrameAllocator, HEAP_START}, modules::vm_core::qemu::serial_stdout::{put_line, put_str}};

use super::{bucket_mgr, util, MAX_HEAP_SIZE};


pub struct BucketedAllocator {
    boot_info: Option<&'static BootInfo>,
}

impl BucketedAllocator {
    pub const fn new() -> Self {
        Self {
            boot_info: None,
        }
    }
    pub unsafe fn init(&mut self, boot_info: &'static BootInfo) {
        self.boot_info = Some(boot_info);

        let mut frame_allocator: BootInfoFrameAllocator<Size4KiB> = BootInfoFrameAllocator::new();
        frame_allocator.init(&boot_info.memory_map);

        let mut page_tables = {
            OffsetPageTable::new(
                {
                    {
                        use x86_64::registers::control::Cr3;
    
                        let (level_4_table_frame, _) = Cr3::read();
    
                        let phys = level_4_table_frame.start_address();
                        let virt = VirtAddr::new(boot_info.physical_memory_offset) + phys.as_u64();
                        let page_table_ptr: *mut x86_64::structures::paging::PageTable = virt.as_mut_ptr();
    
                        &mut *page_table_ptr // unsafe
                    }},VirtAddr::new(boot_info.physical_memory_offset))
        };


        let addr_heap_start = VirtAddr::new(HEAP_START as u64);
        // put_line("Paging...");
        for page in util::page_range_from::<Size4KiB>(addr_heap_start, addr_heap_start + MAX_HEAP_SIZE) {
            #[cfg(feature = "serial_stdout")]
            util::map_page(page, &mut frame_allocator, &mut page_tables);
        }
        // put_line("Paged");        
    }
}

unsafe impl GlobalAlloc for Locked<BucketedAllocator>  {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        let _ = self.lock();
        bucket_mgr::alloc(layout.size(), layout.align())
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: core::alloc::Layout) {
        let _ = self.lock();
        bucket_mgr::dealloc(ptr)
    }
}