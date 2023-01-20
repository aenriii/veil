pub mod vga_text_mode;

#[cfg(feature = "bump_allocator")]
pub mod bump_allocator; // change allocator types at will using feature flags!

#[cfg(feature = "serial_stdout")]
pub mod qemu_serial_stdout; // if we aren't adding STDOUT as a serial port in qemu, there's no point in having this

#[cfg(feature = "allocator")]
pub mod string_writer; // we need an allocator to use string types

#[cfg(feature = "serials")]
pub mod serials;