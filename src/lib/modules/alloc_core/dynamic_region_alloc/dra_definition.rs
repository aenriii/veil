#[macro_export]
macro_rules! dra_definition {
    () => {
        use crate::modules::alloc_core::dynamic_region_alloc::DynamicRegionAllocator;
        use crate::lib::veil_std::util::Locked;
        #[global_allocator]
        #[allow(non_upper_case_globals)]
        pub static Allocator: Locked<DynamicRegionAllocator> = Locked::new(DynamicRegionAllocator::new());
    };
}
