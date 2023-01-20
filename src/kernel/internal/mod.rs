pub(super) mod tables;
pub fn init() {
    tables::init();
}

pub(self) const DOUBLE_FAULT_IST_INDEX: u16 = 0;
pub(super) const HEAP_START: usize = 0x_4444_4444_0000;
pub(super) const HEAP_SIZE: usize = 4 * 1024 * 1024;
pub(super) const HEAP_SIZE_AS_DEBUG_STR: &str = "4Mb";

