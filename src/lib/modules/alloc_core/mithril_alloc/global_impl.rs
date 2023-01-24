use core::alloc::GlobalAlloc;

use crate::std::util::Locked;

use super::MithrilAllocator;



unsafe impl GlobalAlloc for Locked<MithrilAllocator> {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        self.lock().malloc(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: core::alloc::Layout) {
        self.lock().demalloc(ptr, layout)
    }
}