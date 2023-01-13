
#[macro_export]
macro_rules! println {
    () => (print!("\n"));
    ($($arg:tt)*) => (crate::print!("{}\n", format_args!($($arg)*)));
}
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => (
        {
        use core::fmt::Write;
        match $crate::kstd::fmt::WRITER.lock().write_fmt(format_args!($($arg)*)) {
            Ok(_) => {},
            Err(err) => {panic!("PRINT FAILED {}", err)}
        }
    }
);
}