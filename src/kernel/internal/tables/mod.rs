pub(super) mod gdt;
pub(super) mod idt;

pub fn init() {
    gdt::init();
    idt::init();
}