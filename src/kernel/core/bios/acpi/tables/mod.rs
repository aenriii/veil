
mod acpi_table;
mod header;
mod debug;
pub mod rsdt;
use core::str::from_utf8;

pub use acpi_table::AcpiTable;
pub use header::SdtHeader;

use crate::serial_log;

use self::acpi_table::{Rsdt, Xsdt};

pub unsafe fn read_table_at(pointer: *const SdtHeader) -> AcpiTable {
    

    return match from_utf8(&(&*pointer).signature) {
        Ok("RSDT") => return AcpiTable::Rsdt(pointer as *const Rsdt),
        Ok("XSDT") => return AcpiTable::Xsdt(pointer as *const Xsdt),

        Ok(_) => {
            serial_log!("Unknown ACPI table: {}", from_utf8(&(&*pointer).signature).unwrap());

            return AcpiTable::Unknown(pointer);
        }
        Err(_) => {
            serial_log!("Error! ACPI table signature failed utf8 conversion, pointer: {:#x}", pointer as usize);

            return AcpiTable::Invalid;
        }        
    }
}