

pub fn main(boot_info: &'static BootInfo) {
    vga_text_buffer::init();

    vga_text_buffer::write_line("Welcome to Veil v0.0.0!");

    gdt::init();
    idt::init();
    vga_text_buffer::write_log("IDT/ISR/GDT initialized");

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
