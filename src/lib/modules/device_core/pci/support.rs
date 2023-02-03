// use core::arch::asm;

use core::arch::asm;

use crate::serial_log;


pub(crate) fn detect_support() -> PciSupportLevel { unsafe {

    // Check if the PCI BIOS is present
    // For BIOS systems, int 0x1A, AX=0xB101 will tell you if the system uses mechanism #1 or mechanism #2. If this function doesn't exist you can't be sure if the computer supports PCI or not. If it says mechanism #1 is supported you won't know if the memory mapped access mechanism is also supported or not. 

    let mut eax: u32;
    let mut ebx: u32;
    let mut ecx: u32;
    let mut edx: u32;
    
    asm!(
        "mov eax, 0xB101",
        "int 0x1A",
        // fix issue with using bx
        "mov {:e}, ebx",
        out(reg) ebx,
        out("eax") eax,
        out("ecx") ecx,
        out("edx") edx,
    );

    if eax == 0xB101 {
        // PCI BIOS is present
        serial_log!("PCI BIOS is present\n");

        // Check if mechanism #1 is supported
        if ebx & (1 << 0) != 0 {
            serial_log!("PCI BIOS supports mechanism #1\n");

            // Check if mechanism #1 MMIO is supported
            if ebx & (1 << 1) != 0 {
                serial_log!("PCI BIOS supports mechanism #1 MMIO\n");
                return PciSupportLevel::Mechanism1Mmio;
            }

            return PciSupportLevel::Mechanism1;
        }

        // Check if mechanism #2 is supported
        if ebx & (1 << 2) != 0 {
            serial_log!("PCI BIOS supports mechanism #2\n");
            return PciSupportLevel::Mechanism2;
        }

        serial_log!("PCI BIOS does not support any access mechanisms\n");
        return PciSupportLevel::None;
    }

    serial_log!("PCI BIOS is not present\n");




    return PciSupportLevel::None
}}

pub enum PciSupportLevel {
    Mechanism1,
    Mechanism1Mmio,
    Mechanism2,
    None,
}