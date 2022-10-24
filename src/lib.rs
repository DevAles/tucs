#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::tester)]
#![reexport_test_harness_main = "test_main"]

use crate::test_tools::qemu;

pub mod test_tools;

pub trait Test {
    fn run(&self) -> ();
}

impl<T> Test for T
where
    T: Fn()
{
    fn run(&self) {
        print!("> {}...\t", core::any::type_name::<T>());
        self();
        println!("[ok]")
    }
}

pub fn tester(tests: &[&dyn crate::Test]) {
    println!();
    println!("> Running {} tests", tests.len());

    for test in tests {
        test.run();
    }

    qemu::exit(qemu::ExitCode::Ok);
}

pub fn test_panic(info: &core::panic::PanicInfo) -> ! {
    println!("{}", info);

    loop {}
}

#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();

    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    crate::test_panic(info)
}