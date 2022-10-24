pub mod write;

use core::panic::PanicInfo;
use crate::*;

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

pub fn panic(info: &PanicInfo) {
    kernel_println!("{}", info);
}

pub fn run() {
    kernel_println!("{}", WELCOME_MESSAGE);
}