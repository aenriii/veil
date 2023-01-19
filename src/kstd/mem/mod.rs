use bootloader::BootInfo;
use x86_64::{
    structures::paging::{OffsetPageTable, PageTable, Translate, Size4KiB},
    VirtAddr,
};

use actual::BootInfoFrameAllocator;


pub mod pseudo;
pub mod actual;
pub mod alloc;
pub mod heap;

pub mod bump_alloc;

pub(self) static mut BOOT_INFO: Option<&'static BootInfo> = None;
pub(self) static mut PAGE_TABLE: Option<OffsetPageTable> = None;
pub(self) static mut FRAME_ALLOCATOR: Option<BootInfoFrameAllocator<Size4KiB>> = None; 
pub fn init(boot_info: &'static BootInfo) {
    unsafe {
        BOOT_INFO = Some(boot_info);
        PAGE_TABLE = Some(OffsetPageTable::new(
            {
                {
                    use x86_64::registers::control::Cr3;

                    let (level_4_table_frame, _) = Cr3::read();

                    let phys = level_4_table_frame.start_address();
                    let virt = VirtAddr::new(boot_info.physical_memory_offset) + phys.as_u64();
                    let page_table_ptr: *mut PageTable = virt.as_mut_ptr();

                    &mut *page_table_ptr // unsafe
                }
            },
            VirtAddr::new(boot_info.physical_memory_offset),
        ));
        FRAME_ALLOCATOR = Some(BootInfoFrameAllocator::init(&boot_info.memory_map));
    }
    use crate::screen::vga_text_buffer;
    vga_text_buffer::write_log("[mem::init] calling heap::init...");
    unsafe { heap::init(PAGE_TABLE.as_mut().unwrap(), FRAME_ALLOCATOR.as_mut().unwrap()) }.expect("heap initialization failed");
    vga_text_buffer::write_log("[mem::init] heap::init was a success!");

    #[cfg(feature = "mem_test")] {
        use crate::println;
        {
            let addresses = [
                // the identity-mapped vga buffer page
                0xb8000,
                // some code page
                0x201008,
                // some stack page
                0x0100_0020_1a10,
                // virtual address mapped to physical address 0
                boot_info.physical_memory_offset,
            ];

            for &address in &addresses {
                let virt = VirtAddr::new(address);
                let phys = translate_addr(virt);
                // println!("[mem::init] {:?} -> {:?}", virt, phys);
            }
        };
         {
            let page = x86_64::structures::paging::Page::containing_address(VirtAddr::new(0));
            let mut mapper = unsafe { PAGE_TABLE.as_mut().unwrap() };
            let mut frame_allocator = pseudo::EmptyFrameAllocator;
            pseudo::create_example_mapping(page, &mut mapper, &mut frame_allocator);

            // write the string `New!` to the screen through the new mapping
            let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
            // unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e) };
        };
    }
}
pub fn translate_addr(addr: VirtAddr) -> Option<x86_64::PhysAddr> {
    unsafe { PAGE_TABLE.as_ref().unwrap().translate_addr(addr) }
}
