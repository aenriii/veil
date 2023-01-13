use spin::Mutex;



mod qemu_exit;

pub static SUCCESSFUL_TESTS: Mutex<u64> = Mutex::new(0);

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
    use crate::{qemu_exit, serial_println};

    serial_println!("Test Suite running {} test(s)...", tests.len());
    for test in tests {
        test();
    }
    let s_ = SUCCESSFUL_TESTS.lock();
    serial_println!("Ran {} test(s), {} were successful. {}/{}", tests.len(), s_.clone(), tests.len(), s_.clone());
    qemu_exit!(ok);

}

#[test_case]
fn trivial_assertion() {
    use crate::{serial_print, serial_println};
    serial_print!("trivial assertion... ");
    assert_eq!(1, 1);
    serial_println!("[ok]");
    *SUCCESSFUL_TESTS.lock() += 1;
}
#[test_case]
fn WRITER_lock() {
    use crate::{serial_print, serial_println};
    use crate::kstd::fmt::WRITER;
    use core::fmt::Write;
    serial_print!("WRITER lock/write...");
    {
        let _ = write!(&mut WRITER.lock(), "Test");
    }
    serial_println!("[ok]");
    *SUCCESSFUL_TESTS.lock() += 1;

}