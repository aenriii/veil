use lazy_static::lazy_static;
use spin::Mutex;
use uart_16550::SerialPort;


lazy_static! {
    pub static ref SERIAL_STDOUT: Mutex<SerialPort> = {
        let mut serial_p = unsafe {SerialPort::new(0x3F8)};
        serial_p.init();
        Mutex::new(serial_p)
    };
}
pub fn serial_print(args: core::fmt::Arguments) {
    use core::fmt::Write;
    SERIAL_STDOUT.lock().write_fmt(args).expect("SERIAL PRINT FAILED!")
}
#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => {
        $crate::kstd::device::serial::serial_print(format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! serial_println {
    () => ($crate::serial_print!("\n"));
    ($fmt:expr) => ($crate::serial_print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::serial_print!(
        concat!($fmt, "\n"), $($arg)*));
}