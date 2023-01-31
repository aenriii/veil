use x86_64::structures::idt::InterruptStackFrame;

use crate::{kernel::core::{std_vecs::{STDIN, KEYIN}, mem::HEAP_READY}};

use super::{PIC_LOCATION, PICS};

#[cfg(feature = "serial_stdout")]
use crate::serial_println;


#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = PIC_LOCATION,
    Keyboard,
}

pub extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: InterruptStackFrame) {
    // print!(".");
    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Timer as u8);
    }
}
static KI_LOCK: spin::Mutex<()> = spin::Mutex::new(());
pub extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: InterruptStackFrame) {
    // println!("KIH");
    x86_64::instructions::interrupts::without_interrupts(|| {
        
        #[cfg(feature = "ps2_keyboard")] {
            #[cfg(feature = "ps2_keyboard_sync")]
            {
                let _lock = KI_LOCK.lock();
                if let Some(key) = crate::lib::modules::device_core::serial::ps2_keyboard::ps2_keyboard_sync::pull_key() {
                    KEYIN.lock().push_front(key);
                }
            }
            #[cfg(feature = "ps2_keyboard_async")]
            unsafe {
                if HEAP_READY {
                    use x86_64::instructions::port::Port;
                    let mut port = Port::new(0x60);
                    let scancode: u8 = unsafe { port.read() };
                    crate::modules::device_core::serial::ps2_keyboard::ps2_keyboard_async::add_scancode(scancode); 
                } 
            }
        }
        unsafe {PICS.lock().notify_end_of_interrupt(InterruptIndex::Keyboard as u8);}
    });

}
