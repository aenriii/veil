mod port;
mod writer;
mod macros;

use core::fmt::Write;

pub use writer::SerialWriter;

pub fn put_str(s: &str) {
    let mut writer = SerialWriter.lock();
    writer.write_str(s);
}
pub fn put_line(s: &str) {
    let mut writer = SerialWriter.lock();
    writer.write_str(s);
    writer.write_str("\n");
}