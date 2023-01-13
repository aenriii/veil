use spin::Mutex;

macro_rules! vga_screen {
    () => {
        ( 0xb8000 as *mut u8 )
    };
}
#[macro_export]
macro_rules! color {
    (black, black) => {
        0x00
    };
    (black, white) => {
        0x0f
    };
    (white, black) => {
        0xf0
    };
    (white, white) => {
        0xff
    };
    (black, red) => {
        0x04
    };
    (black, green) => {
        0x02
    };
    (black, blue) => {
        0x01
    };
    (white, red) => {
        0x0c
    };
    (white, green) => {
        0x0a
    };
    (white, blue) => {
        0x09
    };
    (red, black) => {
        0x40
    };
    (green, black) => {
        0x20
    };
    (blue, black) => {
        0x10
    };
    (red, white) => {
        0x4c
    };
    (green, white) => {
        0x2a
    };
    (blue, white) => {
        0x19
    };
    (red, red) => {
        0x44
    };
    (green, green) => {
        0x22
    };
    (blue, blue) => {
        0x11
    };
    (red, green) => {
        0x46
    };
}

static mut PTR_X: usize = 0;
static mut PTR_Y: usize = 0;
static mut COLOR: u8 = color!(black, white);
static WRITE_CHAR: Mutex<bool> = Mutex::new(false);
static WRITE_STRING: Mutex<bool> = Mutex::new(false);
const VGA_WIDTH: usize = 80;
const VGA_HEIGHT: usize = 25;


fn ptr_at(x: usize, y: usize) -> *mut u8 {
    unsafe {
        vga_screen!().add((y * 80 + x) * 2)
    }
}
fn ptr() -> *mut u8 {
    unsafe { 
        let p = ptr_at(PTR_X, PTR_Y);
        p
    }
}
fn incr() {
    unsafe {
        PTR_X += 1;
        if PTR_X >= VGA_WIDTH {
            PTR_X = 0;
            PTR_Y += 1;
            if PTR_Y >= VGA_HEIGHT {
                shift_up();
            }
        }
    }
}
fn shift_up() {
    // shift all chars up by a row, ignoring top row
    unsafe {
        for y in 0..(VGA_HEIGHT-1) {
            for x in 0..VGA_WIDTH {
                let mut p = ptr_at(x, y);
                let mut p2 = ptr_at(x, y+1);
                *p = *p2;
                p = p.add(1);
                p2 = p2.add(1);
                *p = *p2;
            }
        }
        // clear bottom row
        for x in 0..VGA_WIDTH {
            let mut p = ptr_at(x, VGA_HEIGHT-1);
            *p = 0;
            p = p.add(1);
            *p = COLOR;
        }
    }
}

pub fn write_char(c: char) {
    let _l = WRITE_CHAR.lock();
    if c == '\r' {
        unsafe {
            PTR_X = 0;
        }
        return;
    }
    if c == '\n' {
        unsafe {
            PTR_Y += 1;
            if PTR_Y >= VGA_HEIGHT {
                shift_up();
            }
            PTR_X = 0; // crlf NOT needed
        }
        return;
    }
    unsafe {
        let mut p = ptr();
        *p = c as u8;
        p = p.add(1);
        *p = COLOR;
        incr();
    }
    
    
}
pub fn write_string(s: &str) {
    let _l = WRITE_STRING.lock();
    for c in s.chars() {
        write_char(c);
    }
}
pub fn write_log(s: &str) {
    unsafe { 
        let old_c = COLOR;
        COLOR = color!(black, green);
        write_line(s);
        COLOR = old_c;
    }
}
pub fn write_error(s: &str) {
    unsafe { 
        let old_c = COLOR;
        COLOR = color!(black, red);
        write_line(s);
        COLOR = old_c;
    }
}
pub fn write_line(s: &str) {
    write_string(s);
    write_char('\n');
}

pub fn set_color(col: u8) {
    unsafe {
        COLOR = col;
    }
}