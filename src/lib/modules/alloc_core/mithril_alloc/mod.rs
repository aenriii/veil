// relies on async/await, cant use it yet.

mod alloc_definition;
mod mithril_allocator;
mod global_impl;
pub(self) mod region;
pub use mithril_allocator::MithrilAllocator;