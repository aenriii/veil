#[macro_export]
macro_rules! print {
    () => {
        {
            #[cfg(feature = "vga_text_mode")] {
                crate::print_text_mode!();
            }
        }
    };
    ($($arg:tt)*) => {
        {
            #[cfg(feature = "vga_text_mode")] {
                crate::print_text_mode!($($arg)*)
            }
        }
    }
}
#[macro_export]
macro_rules! println {
    () => {
        crate::print!("\n")
    };
    ($($arg:tt)*) => {
        crate::print!($($arg)*);
        crate::print!("\n");
    }
}
#[macro_export]
macro_rules! log {
    () => {
        {
            #[cfg(feature = "vga_text_mode")] {
                crate::log_text_mode!();
            }
        }
    };
    ($($arg:tt)*) => {
        {
            #[cfg(feature = "vga_text_mode")] {
                crate::log_text_mode!($($arg)*)
            }
        }
    }
}
#[macro_export]
macro_rules! err {
    () => {
        {
            #[cfg(feature = "vga_text_mode")] {
                crate::error_text_mode!();
            }
        }
    };
    ($($arg:tt)*) => {
        {
            #[cfg(feature = "vga_text_mode")] {
                crate::error_text_mode!($($arg)*)
            }
        }
    }
}