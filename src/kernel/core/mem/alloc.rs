
#[cfg(feature = "bump_allocator")]
crate::bump_allocator_definition!();
#[cfg(feature = "bucket_allocator")]
crate::noalloc_alloc_definition!();