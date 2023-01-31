pub mod tables;

use tables::rsdt;
use tables::AcpiTable;

use crate::kernel::core::bios::acpi;
use crate::serial_log;
pub fn init() { unsafe {
    match tables::read_table_at(rsdt::find_rsdt().expect("No RSDT found")) {
        AcpiTable::Rsdt(mut rsdt) => {
            serial_log!("Found RSDT at {:#x}", rsdt as usize);
            if (*rsdt).header.revision != 0 {
                serial_log!("Claims to be XSDT!");
                rsdt = rsdt as *const
            }


            serial_log!("Looking up pointers...");
            for pointer in (*rsdt).pointers() {
                serial_log!("Found pointer: {:#x}", pointer as usize);
                match tables::read_table_at(pointer) {
                    AcpiTable::Rsdt(_) | AcpiTable::Xsdt(_) => {
                        serial_log!("Found RSDT or XSDT in place of ACPI table, skipping (this shouldnt happen!)");
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
        }
    }
}}