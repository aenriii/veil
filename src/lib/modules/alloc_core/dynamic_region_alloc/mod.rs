mod allocator;
mod alloc_settings;

pub use allocator::DynamicRegionAllocator;
pub(self) use alloc_settings::AllocatorSettings;