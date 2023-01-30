mod vga_text_writer;
mod macros;
pub use vga_text_writer::*;
pub use vga_text_writer::Color::{self, *};
use x86_64::instructions::port::Port;

pub fn init() {
    unsafe {
        // disable blinking cursor
        Port::new(0x3D4).write(0x0A as u8);
        Port::new(0x3D5).write(0x20 as u8);
        // clear screen
        VgaTextWriter.lock().clear_screen(0);

    }
}