use spin::Mutex;

use super::port::QEMU_SERIAL_STDOUT;

pub static SerialWriter: Mutex<SerialWriterT> = Mutex::new(SerialWriterT {});
struct SerialWriterT {

}
impl SerialWriterT {

}
impl core::fmt::Write for SerialWriterT {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        QEMU_SERIAL_STDOUT.lock().write_str(s)
    }
}