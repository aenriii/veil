use bootloader::BootInfo;

use crate::{lib::modules::{self, vga_text_mode::VgaTextWriter}, kernel::{internal::tables, core::mem}, log, color, serial_println, prealloc_log_vga, println};



pub fn main(boot_info: &'static BootInfo) -> ! {
    
    modules::vga_text_mode::init();
    {VgaTextWriter.lock().set_color(color!(Black, White));}
    tables::init();
    mem::init(boot_info);
    {VgaTextWriter.lock().clear_screen(color!(Black, White));}
    println!("Welcome to Veil");
    log!("Refactor OK!");


    loop {}

    // match shell::exec() {
    //     Next::GraphicsVGA => {
    //         unimplemented!("Graphics mode is not implemented yet! :(")
    //     }
    //     Next::HLT => loop {
    //         x86_64::instructions::hlt();
    //     },
    // }
}
