use crate::println;

use super::screen::vga;
mod idt;
mod gdt;

pub(self) mod interrupts;
pub(self) use gdt::*;

macro_rules! hlt {
    () => {
        loop {
            x86_64::instructions::hlt();
        }
    };
}

pub fn kernel_main() {
    vga::write_line("Welcome to Veil v0.0.0!");

    gdt::gdt_init();
    idt::init_idt();
    vga::write_log("Core descriptor tables installed");


    // // test panic screen 
    // {
    //     x86_64::instructions::interrupts::disable();
        
    //     vga::panic_screen();
    // vga::write_line("");
    // vga::write_line("");
    // vga::write_line("");
    // println!("EXCEPTION! DOUBLE FAULT (CODE: {})", 0101);
    // vga::write_line("");
    // vga::write_line("Your computer has encountered a fatal error and will restart in a moment");
    // vga::write_line("");
    // vga::write_line("");
    // vga::write_line("REGISTERS: NOT IMPLEMENTED");
    // vga::write_line("");
    // }
    // hlt!()
    loop {
        // x86_64::instructions::hlt();
    }
}