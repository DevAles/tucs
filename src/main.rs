#![no_std]
#![no_main]

#![feature(custom_test_frameworks)] // To implement a custom test framework
#![test_runner(crate::tests::tester)] // To set a default test runner
#![reexport_test_harness_main = "test_main"] // To change tester name ("main to test_main")

pub mod vga;
pub mod tests;
pub mod kernel;

use core::panic::PanicInfo;

#[panic_handler]
pub fn panic(info: &PanicInfo) -> ! {
    kernel::panic(info);

    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    kernel::run();

    #[cfg(test)]
    test_main();

    loop {}
}