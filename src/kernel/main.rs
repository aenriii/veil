use crate::{lib::modules, log_text_mode, kernel::internal::tables};



pub fn main(boot_info: &'static BootInfo) {
    modules::vga_text_mode::init();
    tables::init();
    log_text_mode!("Welcome to Veil!");

    vga_text_buffer::write_log("Eating ram...");
    mem::init(boot_info);

    match shell::exec() {
        Next::GraphicsVGA => {
            unimplemented!("Graphics mode is not implemented yet! :(")
        }
        Next::HLT => loop {
            x86_64::instructions::hlt();
        },
    }
}
