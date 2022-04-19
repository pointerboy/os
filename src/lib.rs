#![no_std]
#![feature(abi_x86_interrupt)]

pub mod interrupts;
pub mod vga_driver;

pub fn init(){
    interrupts::init_idt();
}
