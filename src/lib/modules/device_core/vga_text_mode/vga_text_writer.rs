use spin::Mutex;


pub static VgaTextWriter: Mutex<VgaTextWriterT> = Mutex::new(VgaTextWriterT {
    pos_x: 0,
    pos_y: 0,
    color_code: 0,
});
const VGA_BUFFER: *mut u8 = 0xb8000 as *mut u8;
const VGA_WIDTH: usize = 80;
const VGA_HEIGHT: usize = 25;
pub struct VgaTextWriterT {
    pos_x: usize,
    pos_y: usize,
    color_code: u8,
}
impl VgaTextWriterT {
    fn bound(&mut self) {
        if self.pos_x >= VGA_WIDTH {
            self.pos_x = 0;
            self.pos_y += 1;
        }
        if self.pos_y >= VGA_HEIGHT {
            self.shift_up();
            self.pos_y = VGA_HEIGHT - 1;
        }
    }
    fn shift_up(&mut self) {
        #[cfg(feature = "serial_stdout")]
        crate::serial_println!("shifting up");
        for ptr in VGA_HEIGHT..(VGA_WIDTH * VGA_HEIGHT-1) {
            unsafe {
                *VGA_BUFFER.offset(ptr as isize * 2) = *VGA_BUFFER.offset((ptr * 2 + VGA_WIDTH * 2) as isize);
                *VGA_BUFFER.offset(ptr as isize * 2 + 1) = *VGA_BUFFER.offset((ptr * 2 + VGA_WIDTH * 2 + 1) as isize);
            }
        }
    }
    pub fn line_up(&mut self) {
        self.pos_y -= 1;
        #[allow(unused_comparisons)]
        if self.pos_y < 0 {
            self.pos_y = 0;
        }
        self.restart_line();
        self.bound();
    }
    pub fn clear_screen(&mut self, color: u8) {
        // replace all characters with 0 and the color code with the given color
        for ptr in 0..(VGA_WIDTH * VGA_HEIGHT) {
            unsafe {
                *VGA_BUFFER.offset(ptr as isize * 2) = 0;
                *VGA_BUFFER.offset(ptr as isize * 2 + 1) = color as u8;
            }
        }
        self.pos_x = 0;
        self.pos_y = 0;
    }
    pub fn restart_line(&mut self) {
        self.pos_x = 0;
        for ptr in 0..VGA_WIDTH {
            unsafe {
                *VGA_BUFFER.offset((self.pos_y * VGA_WIDTH + ptr) as isize * 2) = 0;
                *VGA_BUFFER.offset((self.pos_y * VGA_WIDTH + ptr) as isize * 2 + 1) = self.color_code;
            }
        }
        
    }
    pub fn write_string(&mut self, s: &str) {
        for c in s.chars() {
            self.write_char(c);
        }
    }
    pub fn write_char(&mut self, c: char) {
        match c {
            '\r' => self.pos_x = 0,
            '\n' => {
                self.pos_x = 0;
                self.pos_y += 1;
            }
            c => {
                unsafe {
                    *VGA_BUFFER.offset((self.pos_y * VGA_WIDTH + self.pos_x) as isize * 2) = c as u8;
                    *VGA_BUFFER.offset((self.pos_y * VGA_WIDTH + self.pos_x) as isize * 2 + 1) = self.color_code;
                }
                self.pos_x += 1;
            }
        }
        self.bound();
    }
    pub fn backspace(&mut self) {
        if self.pos_x == 0 {
            if self.pos_y == 0 {
                return;
            }
            self.pos_y -= 1;
            self.pos_x = VGA_WIDTH - 1;
        } else {
            self.pos_x -= 1;
        }
        unsafe {
            *VGA_BUFFER.offset((self.pos_y * VGA_WIDTH + self.pos_x) as isize * 2) = 0;
            *VGA_BUFFER.offset((self.pos_y * VGA_WIDTH + self.pos_x) as isize * 2 + 1) = self.color_code;
        }
    }
    pub fn set_color(&mut self, color: u8) {
        self.color_code = color;
    }
    pub fn get_color(&self) -> u8 {
        self.color_code
    }
    pub fn set_pos(&mut self, x: usize, y: usize) {
        self.pos_x = x;
        self.pos_y = y;
        self.bound();
    }
    pub fn get_pos(&self) -> (usize, usize) {
        (self.pos_x, self.pos_y)
    }

}

pub enum Color {
    Black = 0x00,
    Blue = 0x01,
    Green = 0x02,
    Cyan = 0x03,
    Red = 0x04,
    Magenta = 0x05,
    Brown = 0x06,
    LightGray = 0x07,
    DarkGray = 0x08,
    LightBlue = 0x09,
    LightGreen = 0x0a,
    LightCyan = 0x0b,
    LightRed = 0x0c,
    LightMagenta = 0x0d,
    Yellow = 0x0e,
    White = 0x0f,
}