mod allocator;
mod alloc_settings;
mod dra_definition;
mod global_impl;
pub use allocator::DynamicRegionAllocator;
pub(self) use alloc_settings::AllocatorSettings;