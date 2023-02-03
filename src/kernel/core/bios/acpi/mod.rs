pub mod tables;
mod address;

use tables::*;
use crate::kernel::core::bios::{acpi, ACPI_TABLES};
use crate::serial_log;
pub use address::*;
pub fn init() { unsafe {
    match tables::read_table_at(find_rsdt().expect("No RSDT found")) {
        AcpiTable::Rsdt(mut rsdt) => {
            serial_log!("Found RSDT at {:#x}", rsdt as usize);

            ACPI_TABLES.write().insert("RSDT", &*(rsdt as *const SdtHeader));

            serial_log!("Looking up pointers...");
            for pointer in (*rsdt).pointers() {
                match tables::read_table_at(pointer) {
                    AcpiTable::Rsdt(_) | AcpiTable::Xsdt(_) => {
                        serial_log!("Found RSDT or XSDT in place of ACPI table, skipping (this shouldnt happen!)");
                    },
                    AcpiTable::Fadt(ptr) => {
                        serial_log!("Found FADT");
                        serial_log!("FADT ptr: {:#x}", ptr as usize);
                        // serial_log!("FADT: { :#x?}", *ptr);

                        ACPI_TABLES.write().insert("FADT", &*(ptr as *const SdtHeader));
                    },
                    AcpiTable::Hpet(ptr) => {
                        serial_log!("Found HPET");
                        serial_log!("HPET ptr: {:#x}", ptr as usize);
                        // serial_log!("HPET: { :#x?}", *ptr);

                        ACPI_TABLES.write().insert("HPET", &*(ptr as *const SdtHeader));
                    },
                    AcpiTable::Madt(ptr) => {
                        serial_log!("Found MADT");
                        serial_log!("MADT ptr: {:#x}", ptr as usize);
                        // serial_log!("MADT: { :#x?}", *ptr);

                        ACPI_TABLES.write().insert("MADT", &*(ptr as *const SdtHeader));
                    },
                    AcpiTable::Unknown(ptr) => {
                        match core::str::from_utf8(&(*ptr).signature) {
                            Ok("\0\0\0\0") => {
                                serial_log!("Found null ACPI table!");
                            }
                            Ok(sig) => {
                                serial_log!("Found unknown ACPI table with signature {}", sig);
                            }
                            Err(_) => {
                                serial_log!("Found invalid ACPI table!");
                            }
                        }
                    },
                    AcpiTable::Invalid => {
                        serial_log!("Found invalid ACPI table");
                    }
                }
            }
        },
        AcpiTable::Xsdt(xsdt) => {
            serial_log!("Found XSDT at {:#x}", xsdt as usize);
        },
        AcpiTable::Unknown(ptr) => {
            panic!("Unknown ACPI table found in place of RSDT: {:#x}", ptr as usize)
        },
        AcpiTable::Invalid => {
            panic!("Invalid ACPI table found in place of RSDT")
        },

        AcpiTable::Fadt(_) => {
            panic!("Found FADT in place of RSDT (???)");
        },
        AcpiTable::Hpet(_) => {
            panic!("Found HPET in place of RSDT (???)");
        },
        AcpiTable::Madt(_) => {
            panic!("Found MADT in place of RSDT (???)");
        },
    }
}}