#![no_std]
#![no_main]

use tucs::{println, print};
use tucs::test_tools::qemu;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    println!("[ok]");
    qemu::exit(qemu::ExitCode::Ok);

    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    impossible_operation();
    println!("[test did not panic]");
    qemu::exit(qemu::ExitCode::Err);

    loop {}
}

fn impossible_operation() {
    print!("> should_panic::impossible_operation...\t");
    assert_eq!(1 + 1, 3);
}