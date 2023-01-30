use x86_64::{align_up, PhysAddr};

use crate::{err, kernel::core::{mem::util::phys_to_virt_addr, bios::rsdt::table::{Rsdt, Xsdt}}, print, println};
use self::{pointer::{Rdsp, Rdsp2}, table::SdtHeader};

use super::{ebda, ACPI_TABLES};

pub mod pointer;
pub mod table;
pub mod find;

pub const RSDP_SIGNATURE: &'static [u8; 8] = b"RSD PTR ";

static mut rdsp_ptr: Option<*mut Rdsp> = None;

pub fn init() {
    let sdt_header = find::find_rsdt().expect("RSDT not found");
    let sdt_header = unsafe { &*sdt_header };
    match sdt_header.revision {
        0 => {
            ACPI_TABLES.write().insert("RSDT", sdt_header);
            let rsdt = unsafe { &*(sdt_header as *const SdtHeader as *const Rsdt) };
            // crate::serial_println!("RSDT: {:?}", rsdt);
            for pointer in rsdt.pointers() {
                let sig = unsafe {core::str::from_utf8(&(&*pointer).signature)};
                println!("look! a {:?}!", sig);
                ACPI_TABLES.write().insert(sig.unwrap_or("ERR"), unsafe {(&*pointer)});
            }
            
        },
        _ => {
            ACPI_TABLES.write().insert("XSDT", sdt_header);
            let xsdt = unsafe { &*(sdt_header as *const SdtHeader as *const Xsdt) };
            // crate::serial_println!("XSDT: {:?}", xsdt);

            for pointer in xsdt.pointers() {
                let sig = unsafe {core::str::from_utf8(&(&*pointer).signature)};
                println!("look! a {:?}!", sig);
                ACPI_TABLES.write().insert(sig.unwrap_or("ERR"), unsafe {(&*pointer)});
            }
        },
    }

}

