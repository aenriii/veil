use core::{task::Waker, ops::DerefMut};

use alloc::{sync::Arc, vec::Vec};
use futures_util::task::AtomicWaker;
use lazy_static::lazy_static;
use spin::Mutex;
use x86_64::{VirtAddr, structures::paging::{OffsetPageTable, Size4KiB, FrameAllocator, Page, PageTableFlags, Mapper}};
use conquer_once::spin::Lazy;

use crate::kernel::{core::mem::BootInfoFrameAllocator, self};

use super::region::Region;

static ALLOC_WAKER : Lazy<AtomicWaker> = Lazy::new(|| AtomicWaker::new()); // wakes alloc async loop
static REGIONS: Lazy<Arc<Mutex<Vec<Region>>>> = Lazy::new(|| Arc::new(Mutex::new(Vec::new()))); // holds all unallocated regions
const INITIAL_ALLOC_SIZE: usize = 256 * 1024; // 256 KiB
pub struct MithrilAllocator {
    heap_start: Option<VirtAddr>,
    heap_max_size: usize,
    heap_size: usize,
    heap_next_alloc: usize,
    heap_current_end: usize,

}
impl MithrilAllocator {
    pub const fn new() -> Self {
        Self {
            heap_start: None,
            heap_max_size: 0,
            heap_size: 0,
            heap_next_alloc: 0,
            heap_current_end: 0,
        }
    }
    pub unsafe fn init(&mut self, heap_start: usize, heap_size: usize) {
        self.heap_start = Some(VirtAddr::new(heap_start as u64));
        self.heap_max_size = heap_size;
        
        self.heap_next_alloc = 0;
        self.heap_current_end = 0;
    }
    async fn expand(&mut self, by: usize) {
        // assures the end of the heap is aligned to 4 KiB, then allocates a new region
        // of size `by` and adds it to the regions list on the condition that it ends
        // before the heap's start_addr + max size
        
        let mut end = self.heap_start.unwrap() + self.heap_current_end;
        if end.as_u64() % 4096 != 0 {
            end = end.align_up(4096 as u64);
        }
        let start = end;
        end += by;
        if end.as_u64() <= (self.heap_start.unwrap() + self.heap_max_size).as_u64() {
            // page and map the new region
            let mut framer = kernel::core::mem::FrameAllocator.lock();
            let mut pager = kernel::core::mem::PageTable.lock();
            let page_range = {
                let heap_start = Page::<Size4KiB>::containing_address(start);
                let heap_end = Page::containing_address(end);
                Page::range_inclusive(heap_start, heap_end)
            };
            for page in page_range {
                // we need to have REGIONS locked as little as possible.
                // to do this we should map the page, then lock regions,
                // check if the latest region can be extended, if not, 
                // create a new one, then unlock regions
                let frame = framer.allocate_frame().expect("out of memory");
                let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;
                let map_to_result = unsafe { pager.as_mut().unwrap().map_to(page, frame, flags, framer.deref_mut()) };
                match map_to_result {
                    Ok(f) => {
                        f.flush();
                        let mut regions = REGIONS.lock();
                        if let Some(last) = regions.pop() {
                            if last.can_combine(&Region::new(page.start_address(), page.start_address() + page.size())) {
                                regions.push(last.combine(Region { start: page.start_address(), end: page.start_address() + page.size()}));
                                continue;
                            }
                            regions.push(last);
                        }
                        regions.push(Region::new(page.start_address(), page.start_address() + page.size()));
                    },
                    Err(e) => panic!("failed to map page: {:?}", e)
                }
            }
        }
    }



}