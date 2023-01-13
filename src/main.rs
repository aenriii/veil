#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(abi_x86_interrupt)]
#![feature(panic_can_unwind)]
#![feature(panic_info_message)]
use kstd::screen::vga;
pub mod kstd;
pub mod test;
// #[macro_use]
// extern crate alloc;

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    vga::set_color(color!(black, red));
    println!("PANIC!");
    if let Some(location) = info.location() {
        println!("{}:{}:{}", location.file(), location.line(), location.column());
    }
    if let Some(message) = info.message() {
        println!("Message: {}", message);
    }
    if let Some(payload) = info.payload().downcast_ref::<&str>() {
        println!("Payload: {}", payload);
    }
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    
    #[cfg(test)] {
        test_main();
    }
    #[cfg(not(test))] {
        kstd::kernel::kernel_main();
    }

    
    
    // unsafe { scr.view_at(0xb8000 as *mut u8); } // ASSERTED: scr.view_at works
    loop {}
}