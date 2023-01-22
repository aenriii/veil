use lazy_static::lazy_static;
use spin::Mutex;
use uart_16550::SerialPort;
lazy_static! {
    pub static ref QEMU_SERIAL_STDOUT: Mutex<SerialPort> = {
        let mut serial_p = unsafe {SerialPort::new(0x3F8)};
        serial_p.init();
        Mutex::new(serial_p)
    };
}

