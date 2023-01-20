use spin::Mutex;


pub static VgaTextWriter: Mutex<VgaTextWriterT> = Mutex::new(VgaTextWriterT {
    pos_x: 0,
    pos_y: 0,
    color_code: 0,
});
const VGA_BUFFER: *mut u8 = 0xb8000 as *mut u8;
const VGA_WIDTH: usize = 80;
const VGA_HEIGHT: usize = 25;
struct VgaTextWriterT {
    pos_x: usize,
    pos_y: usize,
    color_code: u8,
}
impl VgaTextWriterT {
    pub fn clear_screen(color: u8) {
        // replace all characters with 0 and the color code with the given color
        for ptr in 0..(VGA_WIDTH * VGA_HEIGHT) {
            unsafe {
                *VGA_BUFFER.offset(ptr as isize * 2) = 0;
                *VGA_BUFFER.offset(ptr as isize * 2 + 1) = color as u8;
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
            '
    
}