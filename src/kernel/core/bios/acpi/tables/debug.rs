use alloc::vec::Vec;

use super::acpi_table::*;

impl core::fmt::Debug for Xsdt {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut pointers: Vec<u64> = self.pointers().iter().map(|&x| x as u64).collect();
        f.debug_struct("Xsdt")
            .field("header", &self.header)
            .field("pointers", &pointers.as_slice())
            .finish()
    }
}
impl core::fmt::Debug for Rsdt {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut pointers: Vec<u32> = self.pointers().iter().map(|&x| x as u32).collect();
        f.debug_struct("Rsdt")
            .field("header", &self.header)
            .field("pointers", &pointers.as_slice())
            .finish()
    }
}