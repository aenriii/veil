

/// A memory region that is managed by the kernel.
#[repr(C, packed)]
#[derive(Copy, Clone, Default, Debug)]
pub struct ManagedRegion {
    pub magic: u32,
    pub bucket: usize,
    // descriptor size + rest = size
    pub size: usize,
    pub align: usize,

    pub prev: usize, // storing pointers :3
    pub next: usize,

    pub data_addr: usize,

    pub self_reference: usize,
}
