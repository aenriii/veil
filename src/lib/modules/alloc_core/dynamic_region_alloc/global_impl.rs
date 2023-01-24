use core::alloc::GlobalAlloc;

use crate::lib::veil_std::util::Locked;

use super::DynamicRegionAllocator;


unsafe impl GlobalAlloc for Locked<DynamicRegionAllocator<'_>> {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        self.lock().internal_alloc(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: core::alloc::Layout) {
        self.lock().internal_dealloc(ptr, layout.size())
    }
}