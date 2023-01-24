use core::{task::Waker, ops::DerefMut};

use alloc::{sync::Arc, vec::Vec};
use futures_util::task::AtomicWaker;
use lazy_static::lazy_static;
use spin::Mutex;
use x86_64::{VirtAddr, structures::paging::{OffsetPageTable, Size4KiB, FrameAllocator, Page, PageTableFlags, Mapper}};
use conquer_once::spin::Lazy;

use crate::{kernel::{core::mem::BootInfoFrameAllocator, self}, std::{util::Locked, desync::{Task}, regions::Region}, modules::{async_core::Executor, alloc_core::libregions::RegionIter}, serial_println};


static ALLOC_WAKER : Lazy<AtomicWaker> = Lazy::new(|| AtomicWaker::new()); // wakes alloc async loop
static REGIONS: Lazy<Arc<Locked<Vec<Region>>>> = Lazy::new(|| Arc::new(Locked::new(Vec::new()))); // holds all unallocated regions
const INITIAL_ALLOC_SIZE: usize = MIN_SAFE_CONTIGUOUS_MEM; // 256 KiB

const MIN_SAFE_MEM_REMAINING: usize = 16 * 1024; // 16 KiB
const MIN_SAFE_CONTIGUOUS_MEM: usize = 4 * 1024; // 4 KiB
pub struct MithrilAllocator {
    heap_start: Option<VirtAddr>,
    heap_max_size: usize,
    heap_size: usize,
    heap_next_alloc: usize,
    heap_current_end: usize,

    public_mutex: Mutex<()>,
}
impl MithrilAllocator {
    pub const fn new() -> Self {
        Self {
            heap_start: None,
            heap_max_size: 0,
            heap_size: 0,
            heap_next_alloc: 0,
            heap_current_end: 0,
            public_mutex: Mutex::new(()),
        }
    }
    pub unsafe fn init(&mut self, heap_start: usize, heap_size: usize) {
        self.heap_start = Some(VirtAddr::new(heap_start as u64));
        self.heap_max_size = heap_size;
        
        self.heap_next_alloc = 0;
        self.heap_current_end = 0;
        self.launch();
    }

    unsafe fn launch(&mut self) {
        self.expand(INITIAL_ALLOC_SIZE);

    }
    fn expand(&mut self, mut by: usize) {
        // assures the end of the heap is aligned to 4 KiB, then allocates a new region
        // of size `by` and adds it to the regions list on the condition that it ends
        // before the heap's start_addr + max size
        if by < self.heap_next_alloc {
            by = self.heap_next_alloc;
        }
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
            let every = 100;
            let mut current = 0;
            for page in page_range {
                current+=1;
                if current % every == 0 {
                    REGIONS.sync();
                }
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
            self.heap_current_end += by;
            self.heap_next_alloc += INITIAL_ALLOC_SIZE;
        }
    }

    fn space_left_in_regions(&self) -> usize {
        let mut space = 0;
        for region in REGIONS.lock().iter() {
            space += region.size();
        }
        space
    }
    fn max_contiguous_space(&self) -> usize {
        REGIONS.sync();
        let mut space = 0;
        let mut current = 0;
        for region in REGIONS.lock().iter() {
            current += region.size();
            if current > space {
                space = current;
            }
        }
        space
    }

    pub fn bound(&mut self) {
        // assures that there is enough memory left in the heap
        
        let mut space = self.space_left_in_regions();
        if space < MIN_SAFE_MEM_REMAINING {
            self.expand(MIN_SAFE_MEM_REMAINING - space);
        }
        space = self.max_contiguous_space();
        if space < MIN_SAFE_MEM_REMAINING {
            self.expand(MIN_SAFE_MEM_REMAINING - space);
        }

    }

    pub(crate) unsafe fn malloc(&mut self, layout: core::alloc::Layout) -> *mut u8 {
        match REGIONS.allocate(layout.size()) {
            Some(ptr) => {
                Executor.borrow_mut().spawn(Task::new(async move {
                    #[cfg(feature = "serial_stdout")]
                    serial_println!("This could cause a deadlock, if something happens just look here.");
                    crate::kernel::core::mem::Allocator.lock().bound();
                }));
                ptr
            },
            None => {
                // we need to expand the heap
                let mut space = self.space_left_in_regions();
                if space < layout.size() {
                    space = self.max_contiguous_space();
                }
                if space < layout.size() {
                    // we need to expand the heap
                    let mut expand_by = layout.size() - space;
                    if expand_by < INITIAL_ALLOC_SIZE {
                        expand_by = INITIAL_ALLOC_SIZE;
                    }
                    self.expand(expand_by);
                }
                match REGIONS.allocate(layout.size()) {
                    Some(ptr) => {
                        ptr
                    },
                    None => {
                        panic!("out of memory");
                    }
                }

            }
        }
    }

    pub(crate) unsafe fn demalloc(&mut self, ptr: *mut u8, layout: core::alloc::Layout) {
        REGIONS.deallocate(ptr, layout.size());
    }
}