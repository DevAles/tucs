#![no_std]
#![no_main]

use core::panic::PanicInfo;

static OPALA: &[u8] = b"Welcome to TUCS!";

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    for (index, &byte) in OPALA.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(index as isize * 2) = byte;
            *vga_buffer.offset(index as isize * 2 + 1) = 0xf;
        }
    }

    loop {}
}
