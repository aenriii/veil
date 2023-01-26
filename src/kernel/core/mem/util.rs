use x86_64::PhysAddr;

use crate::kernel::HEAP_START;

use super::PHYSICAL_OFFSET;


pub unsafe fn phys_to_virt_addr(addr: *mut u8) -> *mut u8 {
    let phys_addr = addr as u64;
    let virt_addr = phys_addr + PHYSICAL_OFFSET.lock().unwrap();
    virt_addr as *mut u8
}