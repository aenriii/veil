use pc_keyboard::{DecodedKey, Keyboard, ScancodeSet1, layouts, HandleControl};
use spin::Mutex;
use x86_64::instructions::port::Port;


static PORT: Mutex<Port<u8>> = Mutex::new(Port::new(0x60));
static KB: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> =
Mutex::new(Keyboard::<layouts::Us104Key, ScancodeSet1>::new(
    HandleControl::Ignore)
);
static PULL_KEY_LOCK: Mutex<()> = Mutex::new(());
pub fn pull_key() -> Option<DecodedKey> {
    let _lock = PULL_KEY_LOCK.lock();
    unsafe {
        let code = PORT.lock().read();
        let mut kb = KB.lock();
        if let Ok(Some(key_event)) = kb.add_byte(code) {
            kb.process_keyevent(key_event)
        } else {
            None
        }
    }
}