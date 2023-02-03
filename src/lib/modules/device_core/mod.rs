
#[cfg(feature = "serial_devices")]
pub mod serial;

#[cfg(feature = "vga_text_mode")]
pub mod vga_text_mode;

pub mod pci;
pub mod cpu;