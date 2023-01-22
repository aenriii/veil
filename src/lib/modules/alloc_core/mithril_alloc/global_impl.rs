use core::alloc::GlobalAlloc;

use crate::std::util::Locked;

use super::MithrilAllocator;



unsafe impl GlobalAlloc for Locked<MithrilAllocator> {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        todo!()
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: core::alloc::Layout) {
        todo!()
    }
}