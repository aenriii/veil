
use linked_list_allocator::LockedHeap;


#[global_allocator]
#[allow(non_upper_case_globals)]
pub static Allocator: LockedHeap = LockedHeap::empty();