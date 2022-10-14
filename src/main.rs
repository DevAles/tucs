#![no_std]
#![no_main]

#![feature(custom_test_frameworks)] // To implement a custom test framework
#![test_runner(crate::tests::tester)] // To set a default test runner
#![reexport_test_harness_main = "test_main"] // To change tester name ("main to test_main")

pub mod kernel;
pub mod tests;

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

// Set crate macros
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::kernel::write::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}