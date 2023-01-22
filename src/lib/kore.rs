use crate::{error_text_mode};

#[panic_handler]
pub fn panic(info: &core::panic::PanicInfo) -> ! {
    error_text_mode!("PANIC!");
    if let Some(message) = info.message() {
        error_text_mode!("Message: {}", message);
    }
    loop {}
}

#[alloc_error_handler]
pub fn alloc_error_handler(layout: alloc::alloc::Layout) -> ! {
    panic!("allocation error: {:?}", layout)
}


