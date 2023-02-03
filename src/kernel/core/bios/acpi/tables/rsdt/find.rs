use x86_64::align_up;

use crate::{err, kernel::core::{mem::util::phys_to_virt_addr, bios::{ebda, acpi::tables::SdtHeader}}, print};
use super::{pointer::{Rdsp, Rdsp2}, RSDP_SIGNATURE, rdsp_ptr};

/// Find RSDT by finding RSDP and then using the address in the RSDP to find the SDT Header
/// 
/// # Safety
/// 
/// This function is listed as safe because the RSDP/T structs are not modified in any way
/// and should otherwise be a constant value for any given system.
/// 
/// The user of this function is responsible for ensuring that the RSDP/T structs are valid
/// and cared for. The user is also responsible for checking the ACPI version and using the
/// correct RSDT
pub fn find_rsdt() -> Option<*mut SdtHeader> { return unsafe {
    if let Some(ptr) = find_rsdp() {
        let rsdp = &*ptr;
        if rsdp.revision == 0 {
            return Some(phys_to_virt_addr(rsdp.rsdt_address as *mut u8) as *mut SdtHeader)
        } else {
            return Some(phys_to_virt_addr(unsafe {
                &*(rsdp as *const Rdsp as *const Rdsp2)
            }.xsdt_address as *mut u8) as *mut SdtHeader)
        }
    } else {
        err!("RSDP not found after scanning EBDA and BIOS, returning None which is almost certain to cause a panic")
    }
    None
}}
pub unsafe fn find_rsdp() -> Option<*mut Rdsp> {
    if let Some(p) = rdsp_ptr {
        return Some(p)
    }
    
    match scan_ebda() {
        Some(rsdp) => {
            rdsp_ptr = Some(rsdp);
            Some(rsdp)
        },
        None => {
            match scan_bios() {
                Some(rsdp) => {
                    rdsp_ptr = Some(rsdp);
                    Some(rsdp)
                },
                None => {   
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