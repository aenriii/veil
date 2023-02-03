use super::SdtHeader;

pub use rsdt::Rsdt;
pub use xsdt::Xsdt;

pub enum AcpiTable {
    Rsdt(*const Rsdt),
    Xsdt(*const Xsdt),
    Unknown(*const SdtHeader),
    Invalid,
}



mod rsdt {
    use core::ptr::addr_of;

    use alloc::vec::Vec;

    use crate::{println, kernel::core::{mem::util::phys_to_virt_addr, bios::acpi::tables::SdtHeader}, serial_log};


    #[repr(packed)]
    #[derive(Copy, Clone)]
    pub struct Rsdt {
        pub header: SdtHeader,
        pub pointers: [u32; 0],
    }

    impl Rsdt {
        pub fn pointers(&self) -> Vec<*const SdtHeader> { unsafe {
            let mut pointers: Vec<*const SdtHeader> = Vec::new();
            let header = &self.header;
            let ptr_count = header.data_len() / 4;
            for ptr_slice in header.data().chunks(4) {
                let ptr = unsafe { core::ptr::read_unaligned(ptr_slice.as_ptr() as *const u32) };
                pointers.push(phys_to_virt_addr(ptr as *mut u8) as *const SdtHeader);
            }
            return pointers;
        }}
    }
}

mod xsdt {
    use core::ptr::addr_of;

    use alloc::vec::Vec;

    use crate::{kernel::core::{bios::acpi::tables::SdtHeader, mem::util::phys_to_virt_addr}, println};

    #[repr(C, packed)]
    #[derive(Copy, Clone)]
    pub struct Xsdt {
        pub header: SdtHeader,
        pub pointers: [u64; 0],
    }
    impl Xsdt {
        pub fn pointers(&self) -> Vec<*const SdtHeader> {
            let mut pointers: Vec<*const SdtHeader> = Vec::new();
            for ptr in 0..(self.header.length - core::mem::size_of::<SdtHeader>() as u32) / 8 { unsafe {
                // find pointer in memory
                let pointer = unsafe { 
                    core::ptr::read_unaligned(
                    {addr_of!(self).add(core::mem::size_of::<SdtHeader>())
                        as *const [u64; 0] as *const u64
                        }.add(ptr as usize * 8)) };
                pointers.push(phys_to_virt_addr(pointer as *mut u8) as *const SdtHeader); 
                // #[cfg(feature = "serial_stdout")]
                println!("pointer: {}", (pointer as *mut u8).add(addr_of!(self) as usize) as u64);
            }}
            pointers
        }
    }
}

