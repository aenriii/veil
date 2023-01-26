use alloc::{vec::Vec, string::String};
use futures_util::{Stream, StreamExt};
use x86_64::VirtAddr;

use crate::{print, modules::device_core::serial::ps2_keyboard::{ps2_keyboard_async::{SCANCODE_QUEUE, ScancodeStream}, KB}, kernel::core::bios::{ebda, self}};

const PS1: &str = "you@veil $> ";

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
                    _ => {}
                }
            }
        }
        let command = stdin.remove(0);
         {
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
                        print!("  find [item]: find specific type of item\n");
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
                        "find" => unsafe {
                            fn help() {
                                print!("Usage: test find <item>\n");
                                print!("Available items:\n");
                                print!("  ebda: find the Extended BIOS Data Area\n");
                                print!("  acpi/rdsp: find the ACPI Root System Description Pointer\n");
                                print!("  acpi/rsdt: find the ACPI Root System Description Table\n");
                            }
                            if args.len() < 3 {
                                help();
                                continue;
                            }
                            match args[2] {
                                "help" | "?"  |
                                "--help" | "-h" => {
                                    help();
                                }
                                "ebda" => {
                                    let ebda = ebda::edba_ptr();
                                    print!("EBDA: {:?}\n", ebda);
                                }
                                "acpi/rdsp" => {
                                    let rdsp = bios::rsdt::find_rsdp();
                                    print!("RSDP: {:?}\n", rdsp);
                                }
                                "acpi/rsdt" => {
                                    let rsdt = bios::rsdt::find_rsdt();
                                    print!("RSDT: {:?}\n", rsdt);
                                }
                                _ => {
                                    print!("Unknown item: {}\n", args[2]);
                                    help();
                                }
                            }

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