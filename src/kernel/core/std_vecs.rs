use alloc::collections::VecDeque;
use lazy_static::lazy_static;
use pc_keyboard::DecodedKey;
use spin::Mutex;


lazy_static! {
    // we need this in lazy_static so that we can stop it from being used before the heap is initialized
    pub static ref STDIN: Mutex<VecDeque<u8>> = Mutex::new(VecDeque::new());
}

lazy_static! {
    // we need this in lazy_static so that we can stop it from being used before the heap is initialized
    pub static ref KEYIN: Mutex<VecDeque<DecodedKey>> = Mutex::new(VecDeque::new());
}
lazy_static! {
    // we need this in lazy_static so that we can stop it from being used before the heap is initialized
    pub static ref STDOUT: Mutex<VecDeque<u8>> = Mutex::new(VecDeque::new());
}
lazy_static! {
    // we need this in lazy_static so that we can stop it from being used before the heap is initialized
    pub static ref STDERR: Mutex<VecDeque<u8>> = Mutex::new(VecDeque::new());
}