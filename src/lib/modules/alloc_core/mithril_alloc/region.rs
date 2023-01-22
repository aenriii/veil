use x86_64::VirtAddr;


pub struct Region {
    pub start: VirtAddr,
    pub end: VirtAddr,
}
impl Region {
    pub fn new(start: VirtAddr, end: VirtAddr) -> Self {
        // zero sized regions are not allowed
        assert!(start < end);
        
        Self { start, end }

    }
    pub fn size(&self) -> usize {
        self.end.as_u64() as usize - self.start.as_u64() as usize
    }
    pub fn contains(&self, addr: VirtAddr) -> bool {
        addr >= self.start && addr <= self.end
    }
    pub fn contains_region(&self, other: &Region) -> bool {
        self.contains(other.start) && self.contains(other.end)
    }
    pub fn overlaps(&self, other: &Region) -> bool {
        self.contains(other.start) || self.contains(other.end)
    }
    pub fn can_combine(&self, other: &Region) -> bool {
        self.overlaps(other) || self.end == other.start || self.start == other.end
    }
    pub fn combine(self, other: Region) -> Region { // consumes self and other, this is useful for the allocator
        let start = if self.start < other.start {
            self.start
        } else {
            other.start
        };
        let end = if self.end > other.end {
            self.end
        } else {
            other.end
        };
        Region::new(start, end)
    }
    pub fn chunk(self, size: usize) -> (Region, Option<Region>) { // you will always get a first region, but the second one is optional.
        if self.size() < size {
            (self, None)
        } else {
            let first = Region::new(self.start, self.start + size as u64 - 1 as u64);
            let second = Region::new(self.start + size as u64, self.end);
            (first, Some(second))
        }
    }

}