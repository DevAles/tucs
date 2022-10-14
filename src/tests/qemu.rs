pub enum ExitCode {
    Ok = 0x10,
    Err = 0x11
}


pub fn exit(code: ExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xF4);
        port.write(code as u8);
    }
}