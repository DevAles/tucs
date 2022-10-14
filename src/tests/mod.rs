#[cfg(test)]
pub fn tester(tests: &[&dyn Fn()]) {
    use crate::println;

    println!("> Running {} tests", tests.len());

    for test in tests {
        test();
    }
}

#[test_case]
fn trivial_assertion() {
    use crate::*;

    print!("> trivial assertion... ");
    assert_eq!(1, 1);
    println!("[ok]");
}