#[macro_export]
macro_rules! bump_allocator_definition {
    () => {
        use crate::lib::modules::bump_allocator::BumpAllocator;
        use crate::lib::veil_std::util::Locked;
        #[global_allocator]
        #[allow(non_upper_case_globals)]
        pub static Allocator: Locked<BumpAllocator> = Locked::new(BumpAllocator::new());
    };
}
