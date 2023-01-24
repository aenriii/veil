
#[cfg(feature = "mithril_allocator")]
pub mod mithril_alloc;

#[cfg(feature = "bump_allocator")]
pub mod bump_allocator;

#[cfg(feature = "libregions")]
pub mod libregions;

#[cfg(feature = "dra_allocator")]
pub mod dynamic_region_alloc;

// #[cfg(feature = "noalloc_allocator")]
pub mod noalloc_alloc;