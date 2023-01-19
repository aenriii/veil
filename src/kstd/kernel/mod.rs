use crate::{mem, screen::vga};
use bootloader::BootInfo;

mod idt;
mod gdt;
pub(self) mod shell;
pub(self) mod interrupts;
pub(self) use gdt::*;

pub static mut USE_STDIN_BY_SHELL: bool = false;

pub fn kernel_main(boot_info: &'static BootInfo) {
    vga::init();

    vga::write_line("Welcome to Veil v0.0.0!");

    gdt::init();
    idt::init();
    vga::write_log("IDT/ISR/GDT initialized");

    vga::write_log("Initializing various memory structures...");
    mem::init(boot_info);
    vga::write_log("OK!");
    vga::write_log("Moving to pseudoshell...");
    unsafe { USE_STDIN_BY_SHELL = true; }

    
    loop {
        x86_64::instructions::hlt();
    }
}