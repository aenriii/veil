use pic8259::ChainedPics;
use spin::Mutex;



pub const PIC_LOCATION: u8 = 0x20;
pub const PIC2_LOCATION: u8 = PIC_LOCATION + 8;
pub static PICS: Mutex<ChainedPics> = Mutex::new(unsafe { ChainedPics::new(PIC_LOCATION, PIC2_LOCATION) });

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = PIC2_LOCATION,
    Keyboard,
}

impl InterruptIndex {
    pub fn as_u8(self) -> u8 {
        self as u8
    }

    pub fn as_usize(self) -> usize {
        usize::from(self.as_u8())
    }
}
