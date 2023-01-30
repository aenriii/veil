use core::ptr::addr_of;

use alloc::vec::Vec;

use crate::{println, kernel::core::mem::util::phys_to_virt_addr};

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct SdtHeader {
    pub signature: [u8; 4],
    pub length: u32,
    pub revision: u8,
    pub checksum: u8,
    pub oem_id: [u8; 6],
    pub oem_table_id: [u8; 8],
    pub oem_revision: u32,
    pub creator_id: u32,
    pub creator_revision: u32,
}
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct Rsdt {
    pub header: SdtHeader,
    // pub pointers: [u32; 0],
}
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct Xsdt {
    pub header: SdtHeader,
    pub pointers: [u64; 0],
}
impl core::fmt::Debug for Xsdt {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut pointers: Vec<u64> = Vec::new();
        for ptr in 0..(self.header.length - core::mem::size_of::<SdtHeader>() as u32) / 8 {
            // find pointer in memory
            let pointer = unsafe { 
                core::ptr::read_unaligned(
                 {addr_of!(self).add(core::mem::size_of::<SdtHeader>())
                     as *const [u64; 0] as *const u64
                     }.add(ptr as usize * 8)) };
            pointers.push(pointer); 
            // #[cfg(feature = "serial_stdout")]
            println!("pointer: {}", pointer);
        }
        f.debug_struct("Xsdt")
            .field("header", &self.header)
            .field("pointers", &pointers.as_slice())
            .finish()
    }
}
impl core::fmt::Debug for Rsdt {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut pointers: Vec<u32> = Vec::new();
        for ptr in 0..(self.header.length - core::mem::size_of::<SdtHeader>() as u32) / 4 {
            // find pointer in memory
            let pointer = unsafe { core::ptr::read_unaligned( {addr_of!(self).add(core::mem::size_of::<SdtHeader>()) as *const [u32; 0] as *const u32 }.add(ptr as usize * 4)) };
            pointers.push(pointer); 
            // #[cfg(feature = "serial_stdout")]
            println!("pointer: {}", pointer);
        }
        f.debug_struct("Rsdt")
            .field("header", &self.header)
            .field("pointers", &pointers.as_slice())
            .finish()
    }
}

impl core::fmt::Debug for SdtHeader {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let s = self.clone();

        let signature = unsafe { core::str::from_utf8_unchecked(&s.signature) };
        let length = s.length;
        let revision = s.revision;
        let checksum = s.checksum;
        let oem_id = unsafe { core::str::from_utf8_unchecked(&s.oem_id) };
        let oem_table_id = unsafe { core::str::from_utf8_unchecked(&s.oem_table_id) };
        let oem_revision = s.oem_revision;
        let creator_id = s.creator_id;
        let creator_revision = s.creator_revision;

        f.debug_struct("SdtHeader")
            .field("signature", &signature)
            .field("length", &length)
            .field("revision", &revision)
            .field("checksum", &checksum)
            .field("oem_id", &oem_id)
            .field("oem_table_id", &oem_table_id)
            .field("oem_revision", &oem_revision)
            .field("creator_id", &creator_id)
            .field("creator_revision", &creator_revision)
            .finish()
    }
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
            println!("pointer: {}", phys_to_virt_addr(pointer as *mut u8) as u64);
        }}
        pointers
    }
}
impl Rsdt {
    pub fn pointers(&self) -> Vec<*const SdtHeader> {
        let mut pointers: Vec<*const SdtHeader> = Vec::new();
        for ptr in 0..(self.header.length - core::mem::size_of::<SdtHeader>() as u32) / 4 { unsafe {
            
            // find pointer in memory
            let pointer = unsafe { core::ptr::read_unaligned( {addr_of!(self).add(core::mem::size_of::<SdtHeader>()) as *const [u32; 0] as *const u32 }.add(ptr as usize * 4)) };
            pointers.push(phys_to_virt_addr(pointer as *mut u8) as *const SdtHeader); 
            // #[cfg(feature = "serial_stdout")]
            
        }}
        pointers
    }
}