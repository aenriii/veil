use alloc::{vec::Vec, string::String};
use futures_util::{Stream, StreamExt};
use lazy_static::lazy_static;
use spin::{Lazy, Mutex};
use x86_64::VirtAddr;

use crate::{print, modules::device_core::serial::ps2_keyboard::{ps2_keyboard_async::{SCANCODE_QUEUE, ScancodeStream}, KB}, kernel::core::bios::{ebda, self, rsdt::init}};

const PS1: &str = "you@veil $> ";

static HISTORY: Lazy<Mutex<Vec<String>>> = Lazy::new(|| Mutex::new(Vec::new()));

pub async fn run() {
    let mut stdin: Vec<String> = Vec::new();
    stdin.push(String::new());
    let mut is_start_of_line = true;
    let mut stream = ScancodeStream::new();
    loop {
        if is_start_of_line {
            print!("{}", PS1);
            is_start_of_line = false;
        }
        while let Some(scancode) = stream.next().await {
            let mut kb = KB.lock();
            if let Ok(Some(key_event)) = kb.add_byte(scancode) {
                match kb.process_keyevent(key_event) {
                    Some(pc_keyboard::DecodedKey::Unicode(c)) => {
                        match c {
                            '\n' => {
                                print!("\n");
                                stdin.push(String::new());
                                is_start_of_line = true;
                                break;
                            }
                            '\x08' => {
                                match stdin.last_mut().unwrap().pop() {
                                    Some(_) => {
                                        print!("\x08 \x08");
                                    }
                                    _ => {}
                                }
                            }
                            _ => {
                                print!("{}", c);
                                stdin.last_mut().unwrap().push(c)
                            }
                        }

                        // #[cfg(feature = "serial_stdout")]
                        // crate::serial_println!("cmd: {}", stdin.last().unwrap());
                    }
                    Some(pc_keyboard::DecodedKey::RawKey(key)) => unsafe {
                        match key {
                            pc_keyboard::KeyCode::ArrowUp => {
                                let mut history = HISTORY.lock();
                                if history.len() > 0 {
                                    let last = history.pop().unwrap();
                                    stdin.last_mut().unwrap().clear();
                                    stdin.last_mut().unwrap().push_str(&last);
                                    print!("\r{}{}", PS1, stdin.last().unwrap());
                                }
                            }
                            pc_keyboard::KeyCode::ArrowDown => {
                                let mut history = HISTORY.lock();
                                if history.len() > 0 {
                                    let last = history.pop().unwrap();
                                    stdin.last_mut().unwrap().clear();
                                    stdin.last_mut().unwrap().push_str(&last);
                                    print!("\r{}{}", PS1, stdin.last().unwrap());
                                }
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
        }
        let command = stdin.remove(0);
        {
            HISTORY.lock().push(command.clone());
            let args: Vec<&str> = command.split_whitespace().collect();
            if args.len() == 0 {
                print!("?\n");
                continue;
            }
            match args[0] {
                "help" => {
                    print!("Available commands:\n");
                    print!("  help: print this help message\n");
                    print!("  echo: print the arguments\n");
                    print!("  clear: clear the screen\n");
                    print!("  test: run various tests\n");
                    print!("  exit: exit the shell\n");
                }
                "echo" => {
                    for arg in args[1..].iter() {
                        print!("{} ", arg);
                    }
                    print!("\n");
                }
                "clear" => {
                    #[cfg(feature = "vga_text_mode")]
                    crate::modules::device_core::vga_text_mode::VgaTextWriter
                        .lock()
                        .clear_screen(crate::color!(Black, White));
                }
                "test" => {
                    fn help() {
                        print!("Available tests:\n");
                        print!("  async: test async core\n");
                        print!("  bios: Run various BIOS init functions.\n");
                        #[cfg(feature = "bucket_allocator")]
                        print!("  alloc [bytes]: test new allocator\n");
                    }
                    if args.len() < 2 {
                        print!("Usage: test <test>\n");
                        help();
                        continue;
                    }
                    match args[1] {
                        "help" | "?"  |
                        "--help" | "-h" => {
                            help();
                        }
                        "async" => {
                            async fn async_fn() {
                                print!("Hello from async!\n");
                                let mut l = core::alloc::Layout::new::<u8>();
                                let mut p = unsafe {alloc::alloc::alloc(
                                    l
                                )};
                                print!("Heap allocations in async! {:?}\n", VirtAddr::new(p as u64));
                                unsafe {
                                    alloc::alloc::dealloc(p, l);
                                }
                            }
                            async_fn().await
                        }
                        #[cfg(feature = "bucket_allocator")]
                        "alloc" => {
                            if args.len() < 3 {
                                print!("Usage: test alloc <bytes>\n");
                                continue;
                            }
                            let bytes = args[2].parse::<usize>().unwrap();
                            let mut l = core::alloc::Layout::from_size_align(bytes, 8).unwrap();
                            let mut p = unsafe {alloc::alloc::alloc(
                                l
                            )};
                            print!("Heap allocations! {:?}\n", VirtAddr::new(p as u64));
                            unsafe {
                                alloc::alloc::dealloc(p, l);
                            }
                        }
                        "bios" => unsafe {
                            init();
                        }
                        _ => {
                            print!("Unknown test: {}\n", args[1]);
                            help();
                        }
                    }
                }
                "exit" => break,
                _ => {
                    print!("Command not found: {}\n", command);
                }
            }
        }
        is_start_of_line = true;
    }
    print!("You have exited the shell! If this was unintentional, lmao");
}