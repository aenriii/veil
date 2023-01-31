
#[macro_export]
macro_rules! serial_log {
    ($($arg:tt)*) => {
        #[cfg(feature = "serial_stdout")]
        {
            $crate::serial_println!($($arg)*);
        }
    };
}