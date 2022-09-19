#![no_std]
#![no_main]

pub mod vga;

use core::panic::PanicInfo;

static WELCOME_MESSAGE: &str = "Welcome to TUCS! \nSystem Running...";

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("{}", WELCOME_MESSAGE);

    loop {}
}
