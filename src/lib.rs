#![no_std]
#![feature(llvm_asm)]
#![feature(abi_x86_interrupt)]

pub mod gdt;
pub mod interrupts;
pub mod print;

pub fn init() {
    // gdt::init();
    interrupts::init_idt();
    unsafe { interrupts::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();
}

pub unsafe fn outb(port: u16, val: u8) {
    llvm_asm!("outb $0, $1" : : "{al}"(val), "{dx}N"(port));
}

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}