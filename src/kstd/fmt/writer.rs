use spin::Mutex;

use crate::screen::{IS_VGA_MODE, vga};


pub static WRITER: Mutex<WriterImpl> = Mutex::new(WriterImpl { is_log: false, is_err: false });   
pub struct WriterImpl {
    is_log: bool,
    is_err: bool,
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
        if self.is_log {
            self.log(string);
            return;
        }
        if self.is_err {
            self.err(string);
            return;
        }
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
    pub fn toggle_log(&mut self, to: bool) {
        self.is_log = to;
        if self.is_log && self.is_err {
            self.is_err = false;
        }
    }
    pub fn toggle_err(&mut self, to: bool) {
        self.is_err = to;
        if self.is_log && self.is_err {
            self.is_log = false;
        }
    }

}
impl core::fmt::Write for WriterImpl {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.write_string(s);
        Ok(())
    }
}