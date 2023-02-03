use super::SdtHeader;

pub use rsdt::Rsdt;
pub use xsdt::Xsdt;
pub use fadt::Fadt;
pub use hpet::Hpet;
pub use madt::Madt;
pub enum AcpiTable {
    Rsdt(*const Rsdt),
    Xsdt(*const Xsdt),
    Fadt(*const Fadt),
    Hpet(*const Hpet),
    Madt(*const Madt),
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
            serial_log!("Found {} pointers", ptr_count);
            for ptr_slice in header.data().chunks(4) {
                let ptr = unsafe { core::ptr::read_unaligned(ptr_slice.as_ptr() as *const u32) };
                pointers.push(phys_to_virt_addr(ptr as *mut u8) as *const SdtHeader);
            }
            serial_log!("returning {} pointers", pointers.len());
            return pointers;
        }}
    }
}

mod xsdt {
    use core::ptr::addr_of;

    use alloc::vec::Vec;

    use crate::{kernel::core::{bios::acpi::tables::SdtHeader, mem::util::phys_to_virt_addr}, println};

    #[repr(packed)]
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

mod fadt {

    use crate::kernel::core::bios::acpi::{tables::SdtHeader, GenericAddressStructure};

    #[repr(packed)]
    #[derive(Copy, Clone, Debug)]
    pub struct Fadt {
        pub header: SdtHeader,
        pub firmware_ctrl: u32,
        pub dsdt: u32,

        pub reserved: u8,

        pub preferred_pm_profile: u8,
        pub sci_int: u16,
        pub smi_cmd: u32,
        pub acpi_enable: u8,
        pub acpi_disable: u8,
        pub s4bios_req: u8,
        pub pstate_cnt: u8,
        pub pm1a_evt_blk: u32,
        pub pm1b_evt_blk: u32,
        pub pm1a_cnt_blk: u32,
        pub pm1b_cnt_blk: u32,
        pub pm2_cnt_blk: u32,
        pub pm_tmr_blk: u32,
        pub gpe0_blk: u32,
        pub gpe1_blk: u32,
        pub pm1_evt_len: u8,
        pub pm1_cnt_len: u8,
        pub pm2_cnt_len: u8,
        pub pm_tmr_len: u8,
        pub gpe0_blk_len: u8,
        pub gpe1_blk_len: u8,
        pub gpe1_base: u8,
        pub cst_cnt: u8,
        pub p_lvl2_lat: u16, // worst case HW latency to enter/exit C2 state
        pub p_lvl3_lat: u16, // worst case HW latency to enter/exit C3 state
        pub flush_size: u16,
        pub flush_stride: u16,
        pub duty_offset: u8,
        pub duty_width: u8,
        pub day_alrm: u8,
        pub mon_alrm: u8,
        pub century: u8,

        pub iapc_boot_arch: u16,

        pub reserved2: u8,
        pub flags: u32,

        pub reset_reg: GenericAddressStructure,

        pub reset_value: u8,
        pub reserved3: [u8; 3],

        pub x_firmware_ctrl: u64,
        pub x_dsdt: u64,

        pub x_pm1a_evt_blk: GenericAddressStructure,
        pub x_pm1b_evt_blk: GenericAddressStructure,
        pub x_pm1a_cnt_blk: GenericAddressStructure,
        pub x_pm1b_cnt_blk: GenericAddressStructure,
        pub x_pm2_cnt_blk: GenericAddressStructure,
        pub x_pm_tmr_blk: GenericAddressStructure,
        pub x_gpe0_blk: GenericAddressStructure,
        pub x_gpe1_blk: GenericAddressStructure,
    }
}

mod hpet {
    use crate::kernel::core::bios::acpi::{tables::SdtHeader, GenericAddressStructure};

    #[repr(packed)]
    #[derive(Copy, Clone)]
    pub struct Hpet {
        pub header: SdtHeader,
        pub hardware_rev_id: u8,
        pub comparator_count: u8,
        pub counter_size_cap: u8,
        pub reserved: u8,
        pub legacy_replacement: u8,
        pub address: GenericAddressStructure,
        pub hpet_number: u8,
        pub minimum_tick: u16,
        pub page_protection: u8,

    }
}

mod madt {
    use crate::kernel::core::bios::acpi::{tables::SdtHeader, GenericAddressStructure};

    #[repr(packed)]
    #[derive(Copy, Clone)]
    pub struct Madt {
        pub header: SdtHeader,
        pub local_apic_addr: u32,
        pub flags: u32,
        pub entries: [u8; 0],
    }
}
