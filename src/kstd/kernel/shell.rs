use alloc::{vec::{Vec}, string::{ToString, String}};

use crate::{device::{keyboard::pull_key}, screen::vga_text_buffer, serial_println};

use super::Next;


static mut STDIN: Vec<&[char]> = Vec::new(); // holds lines of input
static mut STDOUT: Vec<char> = Vec::new();
static mut STDERR: Vec<char> = Vec::new();

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
                            vga_text_buffer::backspace();
                        }
                    }
                    b'\n' => {
                        // newline
                        vga_text_buffer::write_char(character);
                        STDIN.push(&STDOUT);
                        STDOUT = Vec::with_capacity(50);
                    }
                    _ => {
                        STDOUT.push(character);
                        vga_text_buffer::write_char(character);
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
pub fn exec() -> Next {
    loop {
        vga_text_buffer::write_string("you@veil > ");
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
            // echo 
             &['e','c','h','o'] => {
                // echo the arguments
                for arg in args[1..].iter() {
                    for c in arg {
                        unsafe { STDOUT.push(*c) }
                    }
                    unsafe { STDOUT.push(' ') }
                }
                unsafe { STDOUT.push('\n') }
            }
            // clear
            &['c','l','e','a','r'] => {
                vga_text_buffer::clear_screen();
            }
            // help
            &['h','e','l','p'] => {
                unsafe {
                    STDOUT.extend("help - show this message\n".chars());
                    STDOUT.extend("echo - echo the arguments\n".chars());
                    STDOUT.extend("clear - clear the screen\n".chars());
                    STDOUT.extend("graphics - show graphics commands\n".chars());
                    STDOUT.extend("version - show the version\n".chars());
                    STDOUT.extend("quit - quit the shell\n".chars());


                }
            }
            &['q','u','i','t'] => {
                return Next::HLT;
            }
            // graphics
            &['g','r','a','p','h','i','c','s'] => {
                match args.get(1).unwrap_or(&Vec::new()).iter().collect::<String>().as_str() {
                    "vga" => unsafe {
                        // load the VGA module
                        STDOUT.extend("Soon(tm)\n".chars());
                        // return Next::GraphicsVGA;
                    }
                    _ => {
                        unsafe {
                            STDOUT.extend("graphics - show this message\n".chars());
                            STDOUT.extend("graphics vga - load VGA module and start drawing to screen\n".chars());
                        }
                    }
                }
            }
            // version
            &['v','e','r','s','i','o','n'] => {
                unsafe {
                    STDOUT.extend("VeilOS 0.0.1\n".chars());
                }
            }
            _ => {
                vga_text_buffer::write_line("not implemented!");
                serial_println!("not implemented, command: {:?}", args[0]);
            }
        }
        flush_stds();

    }
    Next::HLT
}

fn flush_stds() {
    // flush STDOUT
    for c in unsafe { STDOUT.drain(..) } {
        vga_text_buffer::write_char(c as char);
    }
    // flush STDERR
    for c in unsafe { STDERR.drain(..) } {
        vga_text_buffer::write_char(c as char);
    }
}