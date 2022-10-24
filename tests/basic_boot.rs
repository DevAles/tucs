#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(tucs::tester)]
#![reexport_test_harness_main = "test_main"]

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    tucs::test_panic(info)
}

#[test_case]
fn possible_test() {
    assert_eq!(1, 1)
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();

    loop {}
}