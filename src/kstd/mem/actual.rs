use bootloader::bootinfo::{MemoryMap, MemoryRegionType};
use x86_64::{structures::paging::{PhysFrame, FrameAllocator, Size4KiB, PageSize, OffsetPageTable, Mapper}, PhysAddr};

use super::pseudo::Size512KiB;



pub struct BootInfoFrameAllocator<Size: PageSize> {
    memory_map: &'static MemoryMap,
    next: usize,
    phantom: core::marker::PhantomData<Size>,
}
impl <Size : PageSize> BootInfoFrameAllocator<Size> {
    type Size = Size;
    pub unsafe fn init(memory_map: &'static MemoryMap) -> Self {
        BootInfoFrameAllocator {
            memory_map,
            next: 0,
            phantom: core::marker::PhantomData,
        }
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