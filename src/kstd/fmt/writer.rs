use spin::Mutex;

use crate::kstd::screen::{IS_VGA_MODE, vga};


pub static WRITER: Mutex<WriterImpl> = Mutex::new(WriterImpl {});   
pub struct WriterImpl {  
}
impl WriterImpl {
    pub fn write_byte(&mut self, byte: u8) {
        unsafe {if IS_VGA_MODE {
            vga::write_char(match byte {
                0x20..=0x7e | b'\n' => byte as char,
                _ => 0xfe as char, // ASCII '■'
            });
        }}
    }
    pub fn write_string(&mut self, string: &str) {
        for byte in string.bytes() {
            self.write_byte(byte);
        }
    }
    pub fn log(&mut self, string: &str) {
        unsafe {if IS_VGA_MODE {
            vga::write_log(string);
        }}
    }
    pub fn err(&mut self, string: &str) {
        unsafe {if IS_VGA_MODE {
            vga::write_error(string);
        }}
    }
}
impl core::fmt::Write for WriterImpl {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.write_string(s);
        Ok(())
    }
}