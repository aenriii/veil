// no this, no that...
#![no_std]
#![no_main]

// unstable features
#![feature(custom_test_frameworks)]
#![feature(abi_x86_interrupt)]
#![feature(panic_can_unwind)]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]
#![feature(lint_reasons)]
#![feature(thin_box)]
#![feature(async_fn_in_trait)]

// testing thingies
#![test_runner(crate::test::test_runner)]
#![reexport_test_harness_main = "test_main"]

// linter thingies
#![allow(non_upper_case_globals)]
#![allow(unused)]
#![allow(special_module_name)] // for the "lib" module
#![allow(incomplete_features)]
// import alloc so we can implement memory allocation
extern crate alloc;
// there was some sort of thing we could do to define the entry point using a macro
mod lib;
mod kernel;
mod test;

// pub use lib::veil_std as std;
pub use lib::modules;
// entry point is kernel::main(BootInfo)
use bootloader::entry_point;
entry_point!(kernel::main);

pub mod std {
    pub use crate::lib::veil_std::*;
    #[cfg(feature = "alloc_core")]
    pub use crate::lib::modules::alloc_core as alloc;
    #[cfg(feature = "async_core")]
    pub use crate::lib::modules::async_core as std_async;
    #[cfg(feature = "async_core")]
    pub use crate::lib::modules::async_core::Executor as AsyncExecutor;

    #[cfg(feature = "device_core")]
    pub mod devices {
        pub use crate::lib::modules::device_core as _ref;

        #[cfg(feature = "ps2_keyboard")]
        pub use crate::lib::modules::device_core::serial::ps2_keyboard;
        #[cfg(feature = "vga_text_mode")]
        pub use crate::lib::modules::device_core::vga_text_mode;

    }


    

}