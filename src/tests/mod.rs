pub mod qemu;
pub mod serial;

#[cfg(test)]
pub fn tester(tests: &[&dyn Fn()]) {
    use crate::*;

    println!("> Running {} tests", tests.len());

    for test in tests {
        test();
    }

    qemu::exit(qemu::ExitCode::Ok);
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    use crate::*;

    println!("[failed]\n");
    println!("> Error: {}\n", info);
    qemu::exit(qemu::ExitCode::Err);

    loop {}
}

#[test_case]
fn add() {
    use crate::*;

    print!("> testing add...");
    assert_eq!(1 + 1, 2);
    println!("[ok]")
}