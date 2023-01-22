use core::task::Waker;

use alloc::{sync::Arc, vec::Vec};
use futures_util::task::AtomicWaker;
use lazy_static::lazy_static;
use spin::Mutex;
use x86_64::{VirtAddr, structures::paging::{OffsetPageTable, Size4KiB, FrameAllocator}};
use conquer_once::spin::Lazy;

use crate::kernel::core::mem::BootInfoFrameAllocator;

use super::region::Region;

static ALLOC_WAKER : Lazy<AtomicWaker> = Lazy::new(|| AtomicWaker::new()); // wakes alloc async loop
static REGIONS: Lazy<Arc<Mutex<Vec<Region>>>> = Lazy::new(|| Arc::new(Mutex::new(Vec::new()))); // holds all unallocated regions

pub struct MithrilAllocator<'a> {
    heap_start: VirtAddr,
    heap_max_size: usize,
    heap_size: usize,
    heap_next_alloc: usize,
    heap_current_end: usize,

    pager: Option<OffsetPageTable<'a>>,
    framer: Option<BootInfoFrameAllocator<Size4KiB>>
}
impl <'a> MithrilAllocator<'a> {
    pub fn new() -> Self {
        Self {
            heap_start: VirtAddr::new(0),
            heap_max_size: 0,
            heap_size: 0,
            heap_next_alloc: 0,
            heap_current_end: 0,
            pager: None,
            framer: None
        }
    }
    pub unsafe fn init(&mut self, heap_start: usize, heap_size: usize, pager: OffsetPageTable<'a>, framer: BootInfoFrameAllocator<Size4KiB>) {
        self.heap_start = VirtAddr::new(heap_start as u64);
        self.heap_max_size = heap_size;
        
        self.heap_next_alloc = 0;
        self.heap_current_end = 0;
        self.pager = Some(pager);
        self.framer = Some(framer);
    }
}