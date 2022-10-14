#![no_std]
#![no_main]


#![feature(custom_test_frameworks)]
#![test_runner(crate::tester)]

pub mod vga;

use core::panic::PanicInfo;

static WELCOME_MESSAGE: &str = "> Welcome to TUCS! \n> System Running...\n
 _
| |
| |_ _   _  ___ ___
| __| | | |/ __/ __|
| |_| |_| | (__\\__ \\
 \\__|\\__,_|\\___|___/

         .--.
        |o_o |
        |:_/ |
       //   \\ \\
      (|     | )
     /'\\_   _/`\\
     \\___)=(___/
";

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("{}", WELCOME_MESSAGE);

    loop {}
}


#[cfg(test)]
fn tester(tests: &[&dyn Fn()]) {
    println!("> Running {} tests", tests.len());

    for test in tests {
        test();
    }
}