use crate::{mem, screen::vga_text_buffer};
use bootloader::BootInfo;

mod gdt;
mod idt;
pub(self) mod interrupts;
mod next;
pub(self) mod shell;
pub(self) use next::Next;
pub static mut USE_STDIN_BY_SHELL: bool = false;

pub fn kernel_main(boot_info: &'static BootInfo) {
    vga_text_buffer::init();

    vga_text_buffer::write_line("Welcome to Veil v0.0.0!");

    gdt::init();
    idt::init();
    vga_text_buffer::write_log("IDT/ISR/GDT initialized");

    vga_text_buffer::write_log("Eating ram...");
    mem::init(boot_info);
    unsafe {
        USE_STDIN_BY_SHELL = true;
    }
    match shell::exec() {
        Next::GraphicsVGA => {
            unimplemented!("Graphics mode is not implemented yet! :(")
        }
        Next::HLT => loop {
            x86_64::instructions::hlt();
        },
    }
}
