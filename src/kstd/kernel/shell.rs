use alloc::vec::{Vec};

use crate::{device::{keyboard::pull_key}, screen::vga, serial_println};


static mut STDIN: Vec<&[u8]> = Vec::new(); // holds lines of input
static mut STDOUT: Vec<u8> = Vec::new();
static mut STDERR: Vec<u8> = Vec::new();

pub fn handle_input() {
    // this is exclusively called from an interrupt, we need to do this in a safe way.
    // serial_println!("handling input in shell!");
    match pull_key() {
        Some(Ok(character)) => {
            unsafe {
                match character as u8 {
                    b'\r' => {
                        // push the current line to STDIN
                        STDIN.push(&STDOUT);
                        STDOUT = Vec::with_capacity(50);
                    }
                    b'\x08' => {
                        // backspace
                        if let Some(_) = STDOUT.pop() {
                            vga::backspace();
                        }
                    }
                    b'\n' => {
                        // newline
                        vga::write_char(character);
                        STDIN.push(&STDOUT);
                        STDOUT = Vec::with_capacity(50);
                    }
                    _ => {
                        STDOUT.push(character as u8);
                        vga::write_char(character);
                        // serial_println!("current command: {:?}", STDOUT.iter().map(|c| *c as char).collect::<Vec<char>>());
                    }
                }
            }
        }
        Some(Err(key_code)) => {

            // handle key code
            serial_println!("key code unknown: {:?}", key_code);
            // todo!()
            match key_code {
                _ => {}
            }
        }
       None => {
           // no key pressed
       }           
    }
}
pub fn exec() {
    loop {
        vga::write_string("you@veil > ");
        // wait for input
        while unsafe { STDIN.is_empty() } {
            // wait
            // is there a better way to do this than a busy loop?

            
        }
        // get the first line of input
        let line = unsafe { STDIN.remove(0) };
        let line_chars = line.iter().map(|c| *c as char).collect::<Vec<char>>();
        serial_println!("line: {:?}", line_chars);
        // parse the line
        let mut args = Vec::with_capacity(50);
        let mut arg = Vec::with_capacity(50);
        for c in line_chars {
            if c == ' ' {
                args.push(arg);
                arg = Vec::with_capacity(50);
                // serial_println!("arg: {:?}", arg)
            } else {
                arg.push(c);
                // serial_println!("char in arg: {:?}", c)
            }
        }
        args.push(arg);
        // execute the command
        match args[0].as_slice() {
             &['e','c','h','o'] => {
                // echo the arguments
                for arg in args[1..].iter() {
                    for c in arg {
                        unsafe { STDOUT.push(*c as u8) }
                    }
                    unsafe { STDOUT.push(b' ') }
                }
                unsafe { STDOUT.push(b'\n') }
            }
            _ => {
                vga::write_line("not implemented!");
                serial_println!("not implemented, command: {:?}", args[0]);
            }
        }
        flush_stds();

    }
}

fn flush_stds() {
    // flush STDOUT
    for c in unsafe { STDOUT.drain(..) } {
        vga::write_char(c as char);
    }
    // flush STDERR
    for c in unsafe { STDERR.drain(..) } {
        vga::write_char(c as char);
    }
}