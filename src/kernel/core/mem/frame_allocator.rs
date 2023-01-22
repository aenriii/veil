use bootloader::bootinfo::{MemoryMap, MemoryRegionType};
use x86_64::{structures::paging::{PhysFrame, FrameAllocator, PageSize}, PhysAddr};

lazy_static::lazy_static! {
    static ref BLANK: MemoryMap = MemoryMap::new();
}

pub struct BootInfoFrameAllocator<Size: PageSize> {
    memory_map: &'static MemoryMap,
    next: usize,
    phantom: core::marker::PhantomData<Size>,
}

impl <Size : PageSize> BootInfoFrameAllocator<Size> {
    pub fn new() -> Self {
        BootInfoFrameAllocator { 
            memory_map: &BLANK, 
            next: 0, 
            phantom: core::marker::PhantomData
         }
    }
    pub unsafe fn init(&mut self, mm: &'static MemoryMap) {
        self.memory_map = mm;
    }
    fn usable_frames(&self) -> impl Iterator<Item = PhysFrame<Size>> {
        let regions = self.memory_map.iter();
        let usable_regions = regions.filter(|r| r.region_type == MemoryRegionType::Usable);
        let addr_ranges = usable_regions.map(|r| r.range.start_addr()..r.range.end_addr());
        let frame_addresses = addr_ranges.flat_map(|r| r.step_by(Size::SIZE as usize));
        frame_addresses.map(|addr| PhysFrame::containing_address(PhysAddr::new(addr)))
    }
}
unsafe impl <Size : PageSize> FrameAllocator<Size> for BootInfoFrameAllocator<Size> {
    fn allocate_frame(&mut self) -> Option<PhysFrame<Size>> {
        let frame = self.usable_frames().nth(self.next);
        self.next += 1;
        frame
    }
}