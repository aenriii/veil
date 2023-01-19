

use super::{bump_alloc::BumpAllocator, pseudo::Locked};


#[global_allocator]
#[allow(non_upper_case_globals)]
pub static Allocator: Locked<BumpAllocator> = Locked::new(BumpAllocator::new());