use pic8259::ChainedPics;
use spin::Mutex;
use x86_64::structures::idt::InterruptDescriptorTable;

mod hardware_interrupts;
mod idt_handlers;

use idt_handlers::*;

pub const PIC_LOCATION: u8 = 0x20;
pub const PIC2_LOCATION: u8 = PIC_LOCATION + 8;


pub(self) static PICS: Mutex<ChainedPics> = Mutex::new(unsafe { ChainedPics::new(PIC_LOCATION, PIC2_LOCATION) });
pub(self) static IDT: Mutex<InterruptDescriptorTable> = Mutex::new(InterruptDescriptorTable::new());

pub fn init() {
    unsafe {
        PICS.lock().initialize();
        let mut idt = IDT.lock();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt.double_fault
            .set_handler_fn(double_fault_handler)
            .set_stack_index(crate::kernel::internal::DOUBLE_FAULT_IST_INDEX);
        idt.page_fault.set_handler_fn(page_fault_handler);
        idt.general_protection_fault
            .set_handler_fn(general_protection_fault_handler);
    }
}