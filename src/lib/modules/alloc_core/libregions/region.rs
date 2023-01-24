use core::cell::{Cell, UnsafeCell};

use alloc::{vec, vec::Vec};
use x86_64::VirtAddr;

use crate::lib::veil_std::no_alloc::BoundedIter;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    pub fn can_fit(&self, size: usize) -> bool {
        self.size() >= size
    }
    pub fn can_fit_aligned(&self, size: usize, align: usize) -> bool {
        let start = self.start.align_up(align as u64);
        let end = start + size as u64;
        end <= self.end
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
    pub fn start_addr(&self) -> VirtAddr {
        self.start
    }
    pub fn end_addr(&self) -> VirtAddr {
        self.end
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
    pub fn chunk_aligned(self, size: usize, alignment: usize) -> (Option<BoundedIter<Region, 2>>, Option<Region>) {
        let start = self.start.align_up(alignment as u64);
        let end = start + size as u64;
        let mut iter = BoundedIter::new();
        if end > self.end {
            iter.push(self);
            (Some(iter), None)
        } else {
            let first = Region::new(start, end - 1 as u64);
            let second = Region::new(end, self.end);
            if start == self.start {
                iter.push(first);
                (Some(iter), Some(second))
            } else {
                iter.push(Region::new(self.start, start - 1 as u64));
                iter.push(first);
                (Some(iter), Some(second))
            }
        }
    }

}