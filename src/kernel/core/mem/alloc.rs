
#[cfg(feature = "bump_allocator")]
crate::bump_allocator_definition!();
#[cfg(feature = "mithril_allocator")]
crate::mithril_allocator_definition!();
#[cfg(feature = "dra_allocator")]
crate::dra_definition!();