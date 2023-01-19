use super::{interrupts::PICS, shell, USE_STDIN_BY_SHELL};
use crate::{
    device::keyboard::pull_key, kernel::interrupts::InterruptIndex, screen::vga,
    screen::vga::backspace, *,
};
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode};

static mut IDT: InterruptDescriptorTable = InterruptDescriptorTable::new();
pub fn init() {
    unsafe {
        // PICS
        PICS.lock().initialize();
        // IDT
        IDT.breakpoint.set_handler_fn(breakpoint_handler);
        IDT.double_fault
            .set_handler_fn(double_fault_handler)
            .set_stack_index(super::DOUBLE_FAULT_IST_INDEX);
        IDT.page_fault.set_handler_fn(page_fault_handler);
        IDT.general_protection_fault
            .set_handler_fn(general_protection_fault_handler);

        // Interrupts
        IDT[InterruptIndex::Timer.as_usize()].set_handler_fn(timer_interrupt_handler);
        IDT[InterruptIndex::Keyboard.as_usize()].set_handler_fn(keyboard_interrupt_handler);
        IDT.load();
        PICS.lock().initialize();
        x86_64::instructions::interrupts::enable();
    }
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    //
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}
extern "x86-interrupt" fn double_fault_handler(
    _stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    vga::panic_screen();
    vga::write_line("");
    vga::write_line("");
    vga::write_line("");
    println!("  EXCEPTION! DOUBLE FAULT (CODE: {})", _error_code);
    vga::write_line("");
    vga::write_line("Your computer has encountered a fatal error and will restart in a moment");
    vga::write_line("");
    vga::write_line("");
    vga::write_line("REGISTERS: NOT IMPLEMENTED");
    vga::write_line("");

    loop {}
    // panic!("EXCEPTION: DOUBLE FAULT ({}) \n{:#?}", _error_code, _stack_frame);
}
extern "x86-interrupt" fn page_fault_handler(
    _stack_frame: InterruptStackFrame,
    _error_code: PageFaultErrorCode,
) {
    panic!(
        "EXCEPTION: PAGE FAULT\n{:#?}\n{:#?}",
        _stack_frame, _error_code
    );
}
extern "x86-interrupt" fn general_protection_fault_handler(
    _stack_frame: InterruptStackFrame,
    _error_code: u64,
) {
    panic!(
        "EXCEPTION: GENERAL PROTECTION FAULT\n{:#?}\n{:#?}",
        _stack_frame, _error_code
    );
}
// Interrupts

extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: InterruptStackFrame) {
    // print!(".");
    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Timer.as_u8());
    }
}

extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: InterruptStackFrame) {
    // println!("KIH");
    x86_64::instructions::interrupts::without_interrupts(|| {
        if unsafe { USE_STDIN_BY_SHELL } {
            shell::handle_input();
        } else {
            match pull_key() {
                Some(Ok(character)) => {
                    print!("{}", character);
                    // serial print hex value

                    serial_println!("{}", character as u8)
                }
                Some(Err(key)) => match key {
                    pc_keyboard::KeyCode::Backspace => backspace(),
                    _ => {}
                },
                None => {}
            }
        }
        unsafe {
            PICS.lock()
                .notify_end_of_interrupt(InterruptIndex::Keyboard.as_u8());
        }
    })
}
