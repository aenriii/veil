
pub unsafe fn putb(at: *mut u8, val: u8) {
    *at = val;
}
pub unsafe fn putw(at: *mut u8, val: u16) {
    *at = (val >> 8) as u8;
    *at.add(1) = val as u8;
}
pub unsafe fn putd(at: *mut u8, val: u32) {
    *at = (val >> 24) as u8;
    *at.add(1) = (val >> 16) as u8;
    *at.add(2) = (val >> 8) as u8;
    *at.add(3) = val as u8;
}
pub unsafe fn putq(at: *mut u8, val: u64) {
    *at = (val >> 56) as u8;
    *at.add(1) = (val >> 48) as u8;
    *at.add(2) = (val >> 40) as u8;
    *at.add(3) = (val >> 32) as u8;
    *at.add(4) = (val >> 24) as u8;
    *at.add(5) = (val >> 16) as u8;
    *at.add(6) = (val >> 8) as u8;
    *at.add(7) = val as u8;
}

pub unsafe fn getb(at: *const u8) -> u8 {
    *at
}
pub unsafe fn getw(at: *const u8) -> u16 {
    ((*at as u16) << 8) | (*at.add(1) as u16)
}
pub unsafe fn getd(at: *const u8) -> u32 {
    ((*at as u32) << 24) | ((*at.add(1) as u32) << 16) | ((*at.add(2) as u32) << 8) | (*at.add(3) as u32)
}
pub unsafe fn getq(at: *const u8) -> u64 {
    ((*at as u64) << 56) | ((*at.add(1) as u64) << 48) | ((*at.add(2) as u64) << 40) | ((*at.add(3) as u64) << 32) | ((*at.add(4) as u64) << 24) | ((*at.add(5) as u64) << 16) | ((*at.add(6) as u64) << 8) | (*at.add(7) as u64)
}
