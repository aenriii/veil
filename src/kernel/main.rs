use bootloader::BootInfo;
use pc_keyboard::{KeyCode, DecodedKey::*};
use shell::{PS1_LEN, PS1};
use x86_64::instructions::hlt;

use crate::{lib::modules, kernel::{internal::tables, core::{mem, std_vecs::{STDIN, KEYIN}}, shell::{self, UPDATE_LOCK}}, log, color, serial_println, prealloc_log_vga, println};



pub fn main(boot_info: &'static BootInfo) -> ! {

    #[cfg(feature = "vga_text_mode")]
    modules::device_core::vga_text_mode::init();
    #[cfg(feature = "vga_text_mode")]{modules::device_core::vga_text_mode::VgaTextWriter.lock().set_color(color!(Black, White));}
    tables::init();
    mem::init(boot_info);
    #[cfg(feature = "vga_text_mode")]{modules::device_core::vga_text_mode::VgaTextWriter.lock().clear_screen(color!(Black, White));}
    println!("Welcome to Veil");
    log!("Refactor OK!");

    shell::update();
    loop {
        {
            hlt();
            let mut ki = KEYIN.lock();
            if !{STDIN.lock().is_empty()} {
                shell::update();
            } else {
                #[cfg(feature = "vga_text_mode")]
                if modules::device_core::vga_text_mode::VgaTextWriter.lock().get_pos().0 /* x coord */ < PS1_LEN {
                    let mut v = modules::device_core::vga_text_mode::VgaTextWriter.lock();
                    v.restart_line();
                    v.write_string(PS1)
                }
            }
            if ki.is_empty() {
                continue;
            } 
            // serial_println!("eating key...");

            for k in ki.drain(..) {
                let l = UPDATE_LOCK.lock();
                match k {
                    Unicode(uc) => {
                        STDIN.lock().push_back(uc as u8);
                    }
                    RawKey(key) => {
                        match key {
                            KeyCode::Backspace => {
                                STDIN.lock().pop_front();
                            }
                            _ => {}
                        }
                    }
                }
                core::mem::drop(l);

            }
            
        }
        
    }

    // match shell::exec() {
    //     Next::GraphicsVGA => {
    //         unimplemented!("Graphics mode is not implemented yet! :(")
    //     }
    //     Next::HLT => loop {
    //         x86_64::instructions::hlt();
    //     },
    // }
}
