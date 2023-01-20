#[panic_handler]
pub fn panic(info: &core::panic::PanicInfo) -> ! {
    vga_text_buffer::set_color(color!(black, red));
    println!("PANIC!");
    if let Some(message) = info.message() {
        println!("Message: {}", message);
    }
    loop {}
}

#[alloc_error_handler]
pub fn alloc_error_handler(layout: alloc::alloc::Layout) -> ! {
    panic!("allocation error: {:?}", layout)
}


