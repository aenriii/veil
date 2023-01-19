#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(abi_x86_interrupt)]
#![feature(panic_can_unwind)]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]

use bootloader::BootInfo;
use kstd::screen::vga;

mod kstd;
pub mod test;

pub use kstd::*;

extern crate alloc;

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    vga::set_color(color!(black, red));
    println!("PANIC!");
    if let Some(message) = info.message() {
        println!("Message: {}", message);
    }
    loop {}
}

#[alloc_error_handler]
fn alloc_error_handler(layout: alloc::alloc::Layout) -> ! {
    panic!("allocation error: {:?}", layout)
}

#[no_mangle]
#[allow(unused_variables)] // this is a hack because cargo check often thinks test will always be true
pub extern "C" fn _start(boot_info: &'static BootInfo) -> ! {
    #[cfg(test)] {
        test_main();
    }
    #[cfg(not(test))] {
        kernel::kernel_main(boot_info);
    }
    loop {}
}