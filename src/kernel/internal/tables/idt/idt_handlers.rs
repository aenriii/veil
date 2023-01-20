use x86_64::structures::idt::{PageFaultErrorCode, InterruptStackFrame};

use crate::{err, println};

pub extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    //
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}
pub extern "x86-interrupt" fn double_fault_handler(
    _stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    err!("  EXCEPTION! DOUBLE FAULT (CODE: {})", _error_code);
    loop {}
    // panic!("EXCEPTION: DOUBLE FAULT ({}) \n{:#?}", _error_code, _stack_frame);
}
pub extern "x86-interrupt" fn page_fault_handler(
    _stack_frame: InterruptStackFrame,
    _error_code: PageFaultErrorCode,
) {
    panic!(
        "EXCEPTION: PAGE FAULT\n{:#?}\n{:#?}",
        _stack_frame, _error_code
    );
}
pub extern "x86-interrupt" fn general_protection_fault_handler(
    _stack_frame: InterruptStackFrame,
    _error_code: u64,
) {
    panic!(
        "EXCEPTION: GENERAL PROTECTION FAULT\n{:#?}\n{:#?}",
        _stack_frame, _error_code
    );
}