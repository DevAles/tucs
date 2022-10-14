pub mod qemu;

#[cfg(test)]
pub fn tester(tests: &[&dyn Fn()]) {
    use crate::*;

    println!("> Running {} tests", tests.len());

    for test in tests {
        test();
    }

    qemu::exit(qemu::ExitCode::Ok);
}