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