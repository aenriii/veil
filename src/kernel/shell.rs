use alloc::{vec::Vec, string::String};
use spin::Mutex;

use crate::{serial_println, println, print, color, std::desync::{Task, Executor}};

use super::core::std_vecs::STDIN;


static LINE_BUFFER: Mutex<Vec<char>> = Mutex::new(Vec::new());
pub static PS1: &str = "you@veil:~$ ";
pub static PS1_LEN: usize = 12;
pub static UPDATE_LOCK: Mutex<bool> = Mutex::new(false);
pub fn update() {
    let l = UPDATE_LOCK.lock();
    serial_println!("updating shell");
    
    
    for c in STDIN.lock().drain(..).map(|x| x as char) {
        let mut line_buffer = LINE_BUFFER.lock();
        match c {
            '\r' => {
                // cr
                line_buffer.clear();                
            }
            '\x08' => {
                // backspace
                if let Some(_) = line_buffer.pop() {
                    #[cfg(feature = "vga_text_mode")]
                    {
                        use crate::modules::device_core::vga_text_mode::VgaTextWriter;
                        VgaTextWriter.lock().backspace();
                    }
                }
            }
            '\n' => {
                // newline
                // vga_text_buffer::write_char(character);
                line_buffer.push(c);
                core::mem::drop(line_buffer);
                println!();
                exec();
                
            }
            _ => {
                line_buffer.push(c);
                #[cfg(feature = "vga_text_mode")]
                {
                    use crate::modules::device_core::vga_text_mode::VgaTextWriter;
                    VgaTextWriter.lock().write_char(c);
                }
                // serial_println!("current command: {:?}", STDOUT.iter().map(|c| *c as char).collect::<Vec<char>>());
            }
        }
    }

    serial_println!("done...");
    core::mem::drop(l);
}
fn exec() {
    serial_println!("executing command");
    let mut line_buffer = LINE_BUFFER.lock();
    let line = line_buffer.iter().map(|c| *c).collect::<String>();
    serial_println!("executing: {:?}", line);
    let args = line.split_whitespace().collect::<Vec<&str>>(); 
    match args[0] {
        "help" => {
            println!("help: prints this message");
            println!("echo: prints the arguments");
            println!("clear: clears the screen");
            println!("test: test a certain feature");
        }
        "echo" => {
            for arg in args.iter().skip(1) {
                print!("{} ", arg);
            }
            println!();
        }
        "clear" => {
            #[cfg(feature = "vga_text_mode")]
            {
                use crate::modules::device_core::vga_text_mode::VgaTextWriter;

                let mut vga_tw = VgaTextWriter.lock();
                vga_tw.clear_screen(color!(Black, White));
            }
                
        }
        "test" => {
            if args.len() > 1 {

                match args[1] {
                    "--help"| "-h" | "help" => {
                        println!("test: test a certain feature");
                        println!("test: usage: test [feature]");
                        println!("test: features:");
                        #[cfg(feature = "async_core")]
                        println!("test:     - async");
                    }
                    #[cfg(feature = "async_core")]
                    "async" => {
                        use crate::modules::async_core::Executor;
                        async fn async_num() -> u32 {
                            5
                        }
                        async fn test() {
                            println!("test: async test, num is {}", async_num().await);
                        }
                        let mut e = unsafe {Executor.borrow_mut()};
                        e.spawn(Task::new(test()));
                        e.run();
                    }
                    _ => {
                        println!("test: unknown argument: {}", args[1]);
                    }
                }
            }
        }
        _ => {
            println!("command not found: {}", args[0]);
        }
    }
    line_buffer.clear();

    // todo!()

}