use x86_64::PhysAddr;

use crate::kernel::core::mem::util::phys_to_virt_addr;


pub fn edba_ptr() -> *mut u8 {
    // let edba_ptr = unsafe { *(0x40E as *mut u32) } << (4 as u32);
    let kb_before = unsafe { *(phys_to_virt_addr(0x413 as *mut u8) as *mut u32) };

    return (0x00080000 + (kb_before << 10) as usize) as *mut u8
}