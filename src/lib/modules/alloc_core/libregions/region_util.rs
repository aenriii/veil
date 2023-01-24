
use core::{cmp::max};

use alloc::{vec::Vec, borrow::ToOwned};
use x86_64::VirtAddr;

use crate::std::util::Locked;

use super::region::Region;


pub trait RegionIter {
    fn sync(&self); // assures that all regions are sorted and non-overlapping
    fn find(&self, region: &Region) -> Option<Region>;

    fn allocate(&self, size: usize) -> Option<*mut u8>; // allocates a region of size `size` and returns it
    fn allocate_aligned(&self, size: usize, align: usize) -> Option<*mut u8>; // allocates a region of size `size` and returns it
    fn deallocate(&self, ptr: *mut u8, size: usize); // deallocates a region of size `size` and returns it
    
}
impl RegionIter for Locked<Vec<Region>> {
    fn sync(&self) {
        // TODO: this is a very inefficient way to do this, but it works for now as long as the number of regions is small
        let mut regions = self.lock();
        regions.sort_by(|a, b| a.start_addr().cmp(&b.start_addr()));
        let mut i = 0;
        while i < regions.len() - 1 {
            let a = regions[i].clone();
            let b = regions[i + 1].clone();
            if a.end_addr() > b.start_addr() {
                regions.remove(i);
                regions.remove(i);
                regions.push(Region::new(a.start_addr(), max(a.end_addr(), b.end_addr())));
            }
            i += 1;
        }
    }
    fn find(&self, region: &Region) -> Option<Region> {
        let mut regions = self.lock();
        let mut regions2 = regions.iter().enumerate().filter(|x| (x.1).eq(&region));
        if let Some((i, _)) = regions2.next() {
            Some(regions.remove(i))
        } else {
            None
        }
    }

    fn allocate(&self, size: usize) -> Option<*mut u8> {
        self.allocate_aligned(size, 1)
    }
    fn allocate_aligned(&self, size: usize, align: usize) -> Option<*mut u8> {
        let mut regions2 = self.lock().clone();
        let mut regions2 = regions2.iter().filter(|x| x.can_fit_aligned(size, align));
        let region = regions2.next();
        if let Some(region) = region {
            let region = self.find(region).expect("Region exists but not found");
            let mut regions = self.lock();

            let (remaining, requested) = region.chunk_aligned(size, align);
            if let Some(remaining) = remaining {
                for r in {remaining} {
                    regions.push(({r})); // we can clone here because we aren't losing ownership of the region
                }
            }

            requested.map(|x| x.start_addr().as_u64() as *mut u8)
        } else {
            None
        }

        
    }
    fn deallocate(&self, ptr: *mut u8, size: usize) {
        let mut regions = self.lock();
        regions.push(Region::new(VirtAddr::new(ptr as u64), VirtAddr::new(ptr as u64 + size as u64 - 1)));
        self.sync();
    }
}