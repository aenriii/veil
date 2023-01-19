use alloc::vec::Vec;

use crate::device::keyboard::pull_key;


static mut STDIN: Vec<&[u8]> = Vec::new(); // holds lines of input
static mut STDOUT: Vec<u8> = Vec::new();
static mut STDERR: Vec<u8> = Vec::new();

pub fn handle_input() {
    // this is exclusively called from an interrupt, we need to do this in a safe way.


    /*
    TODO!
     */
    match pull_key() {
        _ => {todo!()}
                

                    
    }
}