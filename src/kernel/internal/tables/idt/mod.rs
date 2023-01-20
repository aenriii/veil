use pic8259::ChainedPics;
use spin::Mutex;

pub(self) mod hardware_interrupts;
pub(self) mod idt_handlers;

pub const PIC_LOCATION: u8 = 0x20;
pub const PIC2_LOCATION: u8 = PIC_LOCATION + 8;


pub(self) static PICS: Mutex<ChainedPics> = Mutex::new(unsafe { ChainedPics::new(PIC_LOCATION, PIC2_LOCATION) });
pub(self) static IDT: Mutex<InterruptDescriptorTable> = Mutex::new(InterruptDescriptorTable::new());

pub fn init() {
    PICS.lock().initialize();
    let idt = IDT.lock();
    
}