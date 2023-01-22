#[macro_export]
macro_rules! mithril_allocator_definition {
    () => {
        use crate::modules::alloc_core::mithril_alloc::MithrilAllocator;
        use crate::lib::veil_std::util::Locked;
        #[global_allocator]
        #[allow(non_upper_case_globals)]
        pub static Allocator: Locked<MithrilAllocator> = Locked::new(MithrilAllocator::new());
    };
}
