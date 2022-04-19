#![no_std]
#![feature(abi_x86_interrupt)]

pub mod interrupts;
pub mod vga_driver;
pub mod gdt;

pub fn init(){
    gdt::init();
    interrupts::init_idt();
}
