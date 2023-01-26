use core::ops::DerefMut;

use lazy_static::__Deref;
use x86_64::{
    structures::paging::{
        mapper::MapToError, FrameAllocator as FA_tr, Mapper, Page, PageTableFlags, Size4KiB,
    },
    VirtAddr,
};

use crate::{
    kernel::{
        internal::{HEAP_SIZE, HEAP_START},
    },
    print, println, serial_println,
};

pub fn init() -> Result<(), MapToError<Size4KiB>> {
    

    Ok(())
}
