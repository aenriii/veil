use bootloader::BootInfo;
use pc_keyboard::{DecodedKey::*, KeyCode};
use shell::{PS1, PS1_LEN};
use x86_64::instructions::hlt;

use crate::{
    color,
    kernel::{
        core::{
            mem,
            std_vecs::{KEYIN, STDIN},
        },
        internal::tables,
        shell::{self, UPDATE_LOCK},
    },
    lib::modules,
    log, prealloc_log_vga, println, serial_println,
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
    println!("Welcome to Veil");

    #[cfg(feature = "async_core")] unsafe {
        log!("Loading core tasks and starting execution engine...");

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
