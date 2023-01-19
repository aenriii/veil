use crate::{mem, screen::vga_text_buffer};
use bootloader::BootInfo;

mod idt;
mod gdt;
mod next;
pub(self) mod shell;
pub(self) mod interrupts;
pub(self) use next::Next;
pub static mut USE_STDIN_BY_SHELL: bool = false;

pub fn kernel_main(boot_info: &'static BootInfo) {
    vga_text_buffer::init();

    vga_text_buffer::write_line("Welcome to Veil v0.0.0!");

    gdt::init();
    idt::init();
    vga_text_buffer::write_log("IDT/ISR/GDT initialized");

    vga_text_buffer::write_log("Initializing various memory structures...");
    mem::init(boot_info);
    vga_text_buffer::write_log("OK!");
    vga_text_buffer::write_log("Moving to pseudoshell...");
    unsafe { USE_STDIN_BY_SHELL = true; }
    vga_text_buffer::write_log("STDIN set to route to shell, moving execution...");
    shell::exec();
    
    loop {
        x86_64::instructions::hlt();
    }
}