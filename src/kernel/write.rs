use core::fmt;

use bitflags::bitflags;
use lazy_static::lazy_static;

use spin::Mutex;
use volatile::Volatile;

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

const ASCII_RANGE_START: u8 = 0x20;
const ASCII_RANGE_END: u8 = 0x7e;

const VGA_BUFFER_START: usize = 0xb8000;

lazy_static! {
    static ref WRITER: Mutex<Writer> = Mutex::new(Writer::new());
}

bitflags! {
    struct COLOR: u8 {
        const BLACK         = 0x0;
        const BLUE          = 0x1;
        const GREEN         = 0x2;
        const CYAN          = 0x3;
        const RED           = 0x4;
        const MAGENTA       = 0x5;
        const BROWN         = 0x6;
        const LIGHT_GRAY    = 0x7;
        const DARK_GRAY     = 0x8;
        const LIGHT_BLUE    = 0x9;
        const LIGHT_GREEN   = 0xA;
        const LIGHT_CYAN    = 0xB;
        const LIGHT_RED     = 0xC;
        const PINK          = 0xD;
        const YELLOW        = 0xE;
        const WHITE         = 0xF;

    }
}

impl COLOR {
    fn new_scheme(foreground: COLOR, background: COLOR) -> u8 {
        background.bits() << 4 | foreground.bits()
    }
}

#[derive(Clone, Copy)]
pub struct Character {
    pub ascii_character: u8,
    pub color: u8,
}

struct Buffer {
    data: [[Volatile<Character>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
    column_position: usize,
    color: u8,
    buffer: &'static mut Buffer,
}

impl Writer {
    pub fn new() -> Self {
        Writer {
            column_position: 0,
            color: COLOR::new_scheme(COLOR::WHITE, COLOR::BLACK),
            buffer: unsafe { &mut *(VGA_BUFFER_START as *mut Buffer) },
        }
    }

    pub fn write_ascii_character(&mut self, ascii_character: u8) {
        match ascii_character {
            b'\n' => self.new_line(),
            _ => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;
                let column = self.column_position;
                let color = self.color;

                self.buffer.data[row][column].write(Character {
                    ascii_character,
                    color,
                });

                self.column_position += 1;
            }
        }
    }

    pub fn write_string(&mut self, string: &str) {
        for character in string.bytes() {
            match character {
                ASCII_RANGE_START..=ASCII_RANGE_END | b'\n' => {
                    self.write_ascii_character(character)
                }
                b'\r' => self.carriage_return(),
                _ => self.write_ascii_character(0xFE),
            }
        }
    }

    pub fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for column in 0..BUFFER_WIDTH {
                let character = self.buffer.data[row][column].read();
                self.buffer.data[row - 1][column].write(character);
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.carriage_return();
    }

    fn carriage_return(&mut self) {
        self.column_position = 0;
    }

    fn clear_row(&mut self, row: usize) {
        let blank = Character {
            ascii_character: b' ',
            color: self.color,
        };

        for column in 0..BUFFER_WIDTH {
            self.buffer.data[row][column].write(blank);
        }
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, string: &str) -> fmt::Result {
        self.write_string(string);
        Ok(())
    }
}

pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}
