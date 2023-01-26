use x86_64::{align_up, PhysAddr};

use crate::{err, kernel::core::mem::util::phys_to_virt_addr, print};
use self::pointer::{Rdsp, Rdsp2};

use super::ebda;

mod pointer;

pub const RSDP_SIGNATURE: &'static [u8; 8] = b"RSD PTR ";

pub unsafe fn find_rsdt() -> Option<*mut u8> {
    if let Some(ptr) = find_rsdp() {
        let rsdp = &*ptr;
        if rsdp.revision == 0 {
            return Some(phys_to_virt_addr(rsdp.rsdt_address as *mut u8))
        } else {
            return Some(phys_to_virt_addr(unsafe {
                // convert rsdp to Rdsp2 by casting
                &*(rsdp as *const Rdsp as *const Rdsp2)
            }.xsdt_address as *mut u8))
        }
    } else {
        err!("RSDP not found after scanning EBDA and BIOS, returning None which is almost certain to cause a panic")
    }
    None
}
pub unsafe fn find_rsdp() -> Option<*mut Rdsp> {
    
    // print!("Scanning EBDA for RSDP... ");
    match scan_ebda() {
        Some(rsdp) => {
            // print!("found at {:p}\n", rsdp);
            Some(rsdp)
        },
        None => {
            // print!("\nnot found, scanning BIOS... ");
            match scan_bios() {
                Some(rsdp) => {
                    // print!("found at {:p}\n", rsdp);
                    Some(rsdp)
                },
                None => {
                    // print!("not found\n");
                    None
                },
            }
        },
    }
}
unsafe fn scan_ebda() -> Option<*mut Rdsp> {

    let mut ptr = ebda::edba_ptr();
    ptr = phys_to_virt_addr(align_up(ptr as u64, 16) as *mut u8);
    let end = ptr.add(1024 * 128);
    while ptr < end {
        if is_here(ptr) {
            return Some(ptr as *mut Rdsp)
        }
        ptr = ptr.add(16);
    }
    None
}
unsafe fn scan_bios() -> Option<*mut Rdsp> {
    let mut ptr = phys_to_virt_addr(0x000E0000 as *mut u8);
    while (ptr) < phys_to_virt_addr(0x000FFFFF as *mut u8) {
        if is_here(ptr) {
            return Some(ptr as *mut Rdsp)
        }
        ptr = ptr.add(16);
    }
    None
}
unsafe fn is_here(ptr: *mut u8) -> bool {
    let ptr = ptr as *mut Rdsp;
    let rsdp = &*ptr;
    return rsdp.signature == *RSDP_SIGNATURE
}