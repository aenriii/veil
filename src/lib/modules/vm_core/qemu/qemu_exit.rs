#[macro_export]
macro_rules! qemu_exit {
    (ok) => {
        unsafe {
            x86_64::instructions::port::Port::new(0xf4).write(0x10 as u32);
        }
    };
    (err) => {
        unsafe {
            x86_64::instructions::port::Port::new(0xf4).write(0x11 as u32);
        }
    }
}
