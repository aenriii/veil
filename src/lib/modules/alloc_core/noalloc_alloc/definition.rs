#[macro_export]
macro_rules! noalloc_alloc_definition {
    () => {
        use crate::modules::alloc_core::noalloc_alloc::BucketedAllocator;
        use crate::lib::veil_std::util::Locked;
        #[global_allocator]
        #[allow(non_upper_case_globals)]
        pub static Allocator: Locked<BucketedAllocator> = Locked::new(BucketedAllocator::new());
    };
}
