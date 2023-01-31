
#[repr(C, packed)]
pub struct Rdsp {
    pub signature: [u8; 8],
    pub checksum: u8,
    pub oem_id: [u8; 6],
    pub revision: u8,
    /// PHYSICAL address of the RSDT
    pub rsdt_address: u32,
}
#[repr(C, packed)]
pub struct Rdsp2 {
    pub signature: [u8; 8],
    pub checksum: u8,
    pub oem_id: [u8; 6],
    pub revision: u8,
    pub rsdt_address: u32,
    pub length: u32,
    pub xsdt_address: u64,
    pub extended_checksum: u8,
    pub reserved: [u8; 3],
}
