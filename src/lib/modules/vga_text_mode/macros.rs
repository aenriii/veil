#[macro_export]
macro_rules! print_text_mode {
    () => {
        {let printer = crate::lib::modules::vga_text_mode::VgaTextWriter.lock();}
    };
    ($($arg:tt)*) => {
    {
        let mut printer = crate::lib::modules::vga_text_mode::VgaTextWriter.lock();
        use ::alloc::string::ToString;
        printer.write_string(format_args!($($arg)*).to_string().as_str());
    }
};
}
#[macro_export]
macro_rules! println_text_mode {
    () => {
        {
            let printer = crate::lib::modules::vga_text_mode::VgaTextWriter.lock();
            printer.write_char('\n');
        }
    };
    ($($arg:tt)*) => {
        {
            let mut printer = crate::lib::modules::vga_text_mode::VgaTextWriter.lock();
            use ::alloc::string::ToString;
            printer.write_string(format_args!($($arg)*).to_string().as_str());
            printer.write_char('\n');
        }
    };
}
#[macro_export]
macro_rules! log_text_mode {
    () => {
        println_text_mode!();
    };
    ($($arg:tt)*) => {
        {
            let color = { crate::lib::modules::vga_text_mode::VgaTextWriter.lock().get_color() };
        { crate::lib::modules::vga_text_mode::VgaTextWriter.lock().set_color(crate::color!(
            Black,
            LightGreen
        )); }
        crate::println_text_mode!($($arg)*);
        { crate::lib::modules::vga_text_mode::VgaTextWriter.lock().set_color(color); }
        }
    };
}
#[macro_export]
macro_rules! error_text_mode {
    () => {
        {
        println_text_mode!();
        }
    };
    ($($arg:tt)*) => {
        {
            let color = { crate::lib::modules::vga_text_mode::VgaTextWriter.lock().get_color() };
        { crate::lib::modules::vga_text_mode::VgaTextWriter.lock().set_color(crate::color!(
            Black,
            LightRed
        )); }
        crate::println_text_mode!($($arg)*);
        { crate::lib::modules::vga_text_mode::VgaTextWriter.lock().set_color(color); }
        }
    };
}
#[macro_export]
macro_rules! color {
    ($back:ident, $fore:ident) => {
        ((crate::lib::modules::vga_text_mode::Color::$back as u8) << 4)
            | crate::lib::modules::vga_text_mode::Color::$fore as u8
    };
}
#[macro_export]
macro_rules! prealloc_log_vga {
    ($str: expr) => {
        {
            let mut x = crate::lib::modules::vga_text_mode::VgaTextWriter.lock();
            x.write_string($str);
            x.write_char('\n');
        }
    };
}