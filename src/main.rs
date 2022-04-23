#![no_std]
#![no_main]

use core::panic::PanicInfo;
use os::init;

mod vga_driver;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    os::hlt_loop();
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    display_println!("Rust OS! ... ");

    init();

    display_println!("It did not crash");
    os::hlt_loop();
}