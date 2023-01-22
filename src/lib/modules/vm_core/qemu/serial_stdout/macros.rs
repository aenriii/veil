#[macro_export]
macro_rules! serial_print {
    () => {
        
    };
    ($($arg:tt)*) => {
        {
            use crate::lib::modules::vm_core::qemu::serial_stdout::SerialWriter;
            use ::core::fmt::Write;
            SerialWriter.lock().write_fmt((format_args!($($arg)*)));
        }
    };
}

#[macro_export]
macro_rules! serial_println {
    () => ($crate::serial_print!("\n"));
    ($fmt:expr) => ($crate::serial_print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::serial_print!(
        concat!($fmt, "\n"), $($arg)*));
}