use pc_keyboard::{Keyboard, layouts, ScancodeSet1, HandleControl};
use spin::Mutex;


#[cfg(feature = "ps2_keyboard_sync")]
pub mod ps2_keyboard_sync;

// #[cfg(feature = "ps2_keyboard_async")]
pub mod ps2_keyboard_async;

pub static KB: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> =
Mutex::new(Keyboard::<layouts::Us104Key, ScancodeSet1>::new(
    HandleControl::Ignore)
);