use bootloader::BootInfo;
use pc_keyboard::{DecodedKey::*, KeyCode};
use x86_64::instructions::hlt;

use crate::{
    color,
    kernel::{
        core::{
            mem,
            std_vecs::{KEYIN, STDIN},
        },
        internal::tables, async_shell    },
    lib::{modules, veil_std::desync::Executor},
    log, prealloc_log_vga, println, serial_println, std::desync::Task, modules::vm_core::qemu::serial_stdout::put_line,
};

pub fn main(boot_info: &'static BootInfo) -> ! {
    #[cfg(feature = "vga_text_mode")]
    modules::device_core::vga_text_mode::init();
    #[cfg(feature = "vga_text_mode")]
    {
        modules::device_core::vga_text_mode::VgaTextWriter
            .lock()
            .set_color(color!(Black, White));
    }
    tables::init();
    mem::init(boot_info);
    #[cfg(feature = "vga_text_mode")]
    {
        modules::device_core::vga_text_mode::VgaTextWriter
            .lock()
            .clear_screen(color!(Black, White));
    }
    #[cfg(feature = "serial_stdout")]
    put_line("out of mem_init, trying allocation-based println.");
    println!("Welcome to Veil");

    #[cfg(feature = "async_core")] unsafe {
        log!("Loading core tasks and starting execution engine...");
        modules::async_core::Executor.borrow_mut().spawn(Task::new(async_shell::run()));
        loop {
            modules::async_core::Executor.borrow_mut().run();
        }
    }
    #[cfg(not(feature = "async_core"))] {
        #[cfg(feature = "serial_stdout")]
        serial_println!("Nothing to do.");
        #[cfg(feature = "vga_text_mode")]
        modules::device_core::vga_text_mode::VgaTextWriter
        .lock()
        .write_str("Nothing to do.");
        loop {
            hlt();
        }
    }
}
