#![no_std]
#![no_main]

pub mod vga;

use core::panic::PanicInfo;
use vga::print;


static WELCOME_MESSAGE: &str = "Welcome to TUCS!";

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    print(WELCOME_MESSAGE);

    loop {}
}
