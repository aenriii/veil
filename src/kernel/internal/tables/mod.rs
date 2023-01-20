pub(super) mod gdt;
pub(super) mod idt;

pub fun init() {
    gdt::init();
    idt::init();
}