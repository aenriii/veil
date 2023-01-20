// no this, no that...
#![no_std]
#![no_main]

// unstable features
#![feature(custom_test_frameworks)]
#![feature(abi_x86_interrupt)]
#![feature(panic_can_unwind)]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]

// testing thingies
#![test_runner(crate::test::test_runner)]
#![reexport_test_harness_main = "test_main"]

// import alloc so we can implement memory allocation
extern crate alloc;

// there was some sort of thing we could do to define the entry point using a macro
mod lib;
mod kernel;
mod test;
// entry point is kernel::main(BootInfo)



