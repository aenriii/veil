use crate::println;

use super::screen::vga;
mod idt;
pub(self) mod interrupts;

macro_rules! hlt {
    () => {
        loop {
            x86_64::instructions::hlt();
        }
    };
}

pub fn kernel_main() {
    vga::write_line("Welcome to Veil v0.0.0!");
    vga::write_log("Opening kernel, installing IDT");
    idt::init_idt();
    vga::write_log("IDT installed");
    // hlt!()
    loop {
        // x86_64::instructions::hlt();
    }
}