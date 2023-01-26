use core::{sync::atomic::Ordering};

use crate::{kernel, modules::vm_core::qemu::serial_stdout::put_line};

use super::{HEAP_START, region::ManagedRegion, size::BUCKETS_AVAIL, MANAGED_MAGIC, buckets::BUCKETS};

const MD_SIZE: usize = core::mem::size_of::<ManagedRegion>();

fn assert_heap() {let _heap_assert = HEAP_START.compare_exchange(0, kernel::HEAP_START, Ordering::SeqCst, Ordering::Relaxed);}

pub fn alloc(size: usize, alignment: usize) -> *mut u8 {
    assert_heap();
    
    let padding = alignment;

    let huge_size = MD_SIZE + padding + size;
    let desc_size = MD_SIZE + padding;

    let bucket = BUCKETS_AVAIL.iter().position(
        |&x| x as usize >= huge_size
    );
    
    let alloc_size = bucket.map_or(huge_size, |x| BUCKETS_AVAIL[x] as usize); 
    let bucket = bucket.unwrap_or(BUCKETS_AVAIL.len());

    let descriptor_addr = pop_from_bucket(bucket, alloc_size).unwrap_or_else(|| HEAP_START.fetch_add(alloc_size, Ordering::SeqCst));

    let descriptor = unsafe { &mut *(descriptor_addr as *mut ManagedRegion) };
    
    descriptor.magic = MANAGED_MAGIC;
    descriptor.bucket = bucket;
    descriptor.size = alloc_size;
    descriptor.align = alignment;
    descriptor.prev = 0;
    descriptor.next = 0;
    descriptor.self_reference = 0;
    descriptor.data_addr = descriptor_addr + desc_size & !(alignment - 1);

    let descriptor_store = descriptor.data_addr - core::mem::size_of::<usize>();
    unsafe { *(descriptor_store as *mut usize) = descriptor_addr };
    
    descriptor.data_addr as *mut u8
}

pub fn dealloc(ptr: *mut u8) {
    assert_heap();

    let descriptor_addr_store = ptr as usize - core::mem::size_of::<usize>();
    let descriptor_addr = unsafe { *(descriptor_addr_store as *mut usize) };
    let descriptor = unsafe { &mut *(descriptor_addr as *mut ManagedRegion) };
    assert!(descriptor.magic == MANAGED_MAGIC, "invalid magic");
    // clean
    descriptor.magic = 0; // no more meow :(

    // quick heap check
    if HEAP_START.compare_exchange(descriptor_addr + descriptor.size, descriptor_addr, Ordering::SeqCst, Ordering::Relaxed).is_ok() {
        return; // this block was the last heap-allocated one and can just be backed up
    } 

    push_to_bucket(descriptor);
}


fn pop_from_bucket(idx: usize, size: usize) -> Option<usize> {
    if idx == BUCKETS.len() {
        // TODO: Dynamic high-size allocation
        
        return None;
    }
    let head_p = BUCKETS[idx].start.load(Ordering::Acquire);
    if head_p != 0 {
        let head = unsafe { &mut *(head_p as *mut ManagedRegion) };
        if BUCKETS[idx].start.compare_exchange(head_p, head.next, Ordering::AcqRel, Ordering::Relaxed).is_ok() {
            if head.next != 0 {
                let next = unsafe { &mut *(head.next as *mut ManagedRegion) };
                next.prev = 0;
            } else {
                BUCKETS[idx].end.store(0, Ordering::SeqCst);
            }
            return Some(head_p);
        }
    }
    

    None
}

fn push_to_bucket(descr: &mut ManagedRegion) {
    let d_addr = descr as *mut ManagedRegion as usize;
    loop {
        // :3
        let prev_bucket = BUCKETS[descr.bucket].end.load(Ordering::Acquire);

        descr.prev = prev_bucket;
        descr.next = 0;
        if BUCKETS[descr.bucket].end.compare_exchange(prev_bucket, d_addr, Ordering::AcqRel, Ordering::Relaxed).is_ok() {
            if prev_bucket != 0 {
                let p_desc = unsafe { &mut *(prev_bucket as *mut ManagedRegion) };
                p_desc.next = d_addr;
            } else {
                BUCKETS[descr.bucket].start.store(d_addr, Ordering::SeqCst);
            }
            return;
        }

    }
}