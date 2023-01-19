use lazy_static::lazy_static;
use pc_keyboard::{Keyboard, layouts, ScancodeSet1, HandleControl, DecodedKey, KeyCode};
use spin::Mutex;
use x86_64::instructions::port::Port;

lazy_static! {
    static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> =
            Mutex::new(Keyboard::<layouts::Us104Key, ScancodeSet1>::new(
                HandleControl::Ignore)
            );
}
lazy_static! {
    static ref KEYBOARD_PORT: Mutex<Port<u8>> = Mutex::new(Port::new(0x60));
}
pub fn pull_key() -> Option<Result<char, KeyCode>> {
    let mut keyboard = KEYBOARD.lock();
    let mut port = KEYBOARD_PORT.lock();
    // vga::write_log("pull_key");
    let code = unsafe {port.read()};
    // vga::write_log("pull_key");
    if let Ok(Some(key_event)) = keyboard.add_byte(code) {
        if let Some(key) = keyboard.process_keyevent(key_event) {
            // vga::write_log("pull_key");
            return match key {
                DecodedKey::Unicode(character) => Some(Ok(character)),
                DecodedKey::RawKey(key) => Some(Err(key)),
            }
        }
    } 
    None
}