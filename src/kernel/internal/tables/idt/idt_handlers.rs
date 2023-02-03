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
    panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", _stack_frame);
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
pub extern "x86-interrupt" fn invalid_opcode_handler(_stack_frame: InterruptStackFrame) {
    panic!("EXCEPTION: INVALID OPCODE\n{:#?}", _stack_frame);
}
pub extern "x86-interrupt" fn stack_segment_fault_handler(
    _stack_frame: InterruptStackFrame,
    _error_code: u64,
) {
    panic!(
        "EXCEPTION: STACK SEGMENT FAULT\n{:#?}\n{:#?}",
        _stack_frame, _error_code
    );
}
pub extern "x86-interrupt" fn divide_by_zero_handler(_stack_frame: InterruptStackFrame) {
    panic!("EXCEPTION: DIVIDE BY ZERO\n{:#?}", _stack_frame);
}
pub extern "x86-interrupt" fn overflow_handler(_stack_frame: InterruptStackFrame) {
    panic!("EXCEPTION: OVERFLOW\n{:#?}", _stack_frame);
}
pub extern "x86-interrupt" fn bound_range_exceeded_handler(_stack_frame: InterruptStackFrame) {
    panic!("EXCEPTION: BOUND RANGE EXCEEDED\n{:#?}", _stack_frame);
}
pub extern "x86-interrupt" fn invalid_tss_handler(_stack_frame: InterruptStackFrame, _error_code: u64) {
    panic!(
        "EXCEPTION: INVALID TSS\n{:#?}\n{:#?}",
        _stack_frame, _error_code
    );
}
pub extern "x86-interrupt" fn segment_not_present_handler(
    _stack_frame: InterruptStackFrame,
    _error_code: u64,
) {
    panic!(
        "EXCEPTION: SEGMENT NOT PRESENT\n{:#?}\n{:#?}",
        _stack_frame, _error_code
    );
}
pub extern "x86-interrupt" fn x87_floating_point_handler(_stack_frame: InterruptStackFrame) {
    panic!("EXCEPTION: X87 FLOATING POINT\n{:#?}", _stack_frame);
}
pub extern "x86-interrupt" fn alignment_check_handler(
    _stack_frame: InterruptStackFrame,
    _error_code: u64,
) {
    panic!(
        "EXCEPTION: ALIGNMENT CHECK\n{:#?}\n{:#?}",
        _stack_frame, _error_code
    );
}
pub extern "x86-interrupt" fn machine_check_handler(_stack_frame: InterruptStackFrame) -> ! {
    panic!("EXCEPTION: MACHINE CHECK\n{:#?}", _stack_frame);
}
pub extern "x86-interrupt" fn simd_floating_point_handler(_stack_frame: InterruptStackFrame) {
    panic!("EXCEPTION: SIMD FLOATING POINT\n{:#?}", _stack_frame);
}
pub extern "x86-interrupt" fn virtualization_handler(_stack_frame: InterruptStackFrame) {
    panic!("EXCEPTION: VIRTUALIZATION\n{:#?}", _stack_frame);
}
pub extern "x86-interrupt" fn security_exception_handler(
    _stack_frame: InterruptStackFrame,
    _error_code: u64,
) {
    panic!(
        "EXCEPTION: SECURITY EXCEPTION\n{:#?}\n{:#?}",
        _stack_frame, _error_code
    );
}
