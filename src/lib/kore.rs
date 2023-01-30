use crate::{error_text_mode, modules::device_core::vga_text_mode::{VgaTextWriter, VgaTextWriterT}, color, kernel::PANIC};

#[panic_handler]
pub fn panic(info: &core::panic::PanicInfo) -> ! {
    unsafe {
        PANIC = true;
    }
    
    if let Some(mut writer) = VgaTextWriter.try_lock() {
        writer.set_color(color!(Red, White));
        writer.clear_screen(color!(Red, White));
        writer.set_color(color!(White, Red));
        writer.write_string("PANIC!\n");
        if let Some(message) = info.message() {
            writer.write_string("Message: ");
            use core::fmt::Write;
            write!(*writer, "{}\n", message).unwrap();
        }
        if let Some(location) = info.location() {
            writer.write_string("Location: ");
            use core::fmt::Write;
            write!(*writer, "{}\n", location).unwrap();
        }
        loop {}
    } else {
        // force get new writer
        let mut writer = unsafe {VgaTextWriterT::new()};
        writer.set_color(color!(Red, White));
        writer.clear_screen(color!(Red, White));
        writer.set_color(color!(White, Red));
        writer.write_string("[DEADLOCK AVOIDED] PANIC!\n");
        if let Some(message) = info.message() {
            writer.write_string("Message: ");
            use core::fmt::Write;
            write!(writer, "{}\n", message).unwrap();
        }
        if let Some(location) = info.location() {
            writer.write_string("Location: ");
            use core::fmt::Write;
            write!(writer, "{}\n", location).unwrap();
        }
        loop {}
    }
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


