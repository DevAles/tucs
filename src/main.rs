#![no_std]
#![no_main]
#![feature(custom_test_frameworks)] // To implement a custom test framework
#![test_runner(crate::tests::tester)] // To set a default test runner
#![reexport_test_harness_main = "test_main"] // To change tester name ("main to test_main")

pub mod kernel;
pub mod tests;

#[cfg(not(test))]
#[panic_handler]
pub fn panic(info: &core::panic::PanicInfo) -> ! {
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

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        $crate::tests::serial::_print(format_args!($($arg)*))
    };
}

#[macro_export]
macro_rules! println {
    () => {
        $crate::print!("\n")
    };

    ($fmt:expr) => {
        $crate::print!(concat!($fmt, "\n"))
    };

    ($fmt:expr, $($arg:tt)*) => {
        $crate::print!(concat!($fmt, "\n"), $($arg)*)
    };
}

#[macro_export]
macro_rules! kernel_print {
    ($($arg:tt)*) => {
        $crate::kernel::write::_print(format_args!($($arg)*))
    };
}

#[macro_export]
macro_rules! kernel_println {
    () => {
        $crate::kernel_print!("\n")
    };

    ($($arg:tt)*) => {
        $crate::kernel_print!("{}\n", format_args!($($arg)*))
    };
}