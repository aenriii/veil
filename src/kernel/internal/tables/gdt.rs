use lazy_static::lazy_static;
use x86_64::{
    structures::{
        tss::TaskStateSegment, 
        gdt::{
            GlobalDescriptorTable, 
            Descriptor, 
            SegmentSelector
        }
    },
    VirtAddr
};

use crate::kernel::internal::DOUBLE_FAULT_IST_INDEX;


lazy_static! {
    static ref TSS: TaskStateSegment = {
        let mut tss = TaskStateSegment::new();
        tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = {
            const STACK_SIZE: usize = 4096 * 5;
            static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

            let stack_start = VirtAddr::from_ptr(unsafe { &STACK });
            let stack_end = stack_start + STACK_SIZE;
            stack_end
        };
        tss
    };
}
lazy_static! {
    static ref GDT: (GlobalDescriptorTable, Selectors) = {
        let mut gdt = GlobalDescriptorTable::new();
        let cs = gdt.add_entry(Descriptor::kernel_code_segment());
        let ts = gdt.add_entry(Descriptor::tss_segment(&TSS));
        (gdt, Selectors {
            code_selector: cs,
            tss_selector: ts,
        })
    };
}
struct Selectors {
    code_selector: SegmentSelector,
    tss_selector: SegmentSelector,
}

pub fn init() {
    use x86_64::instructions::tables::load_tss;
    use x86_64::instructions::segmentation::{CS, Segment};
    
    GDT.0.load();
    unsafe {
        CS::set_reg(GDT.1.code_selector);
        load_tss(GDT.1.tss_selector);
    }
}