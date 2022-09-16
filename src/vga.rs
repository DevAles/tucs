use bitflags::bitflags;

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

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

struct Character {
    ascii_character: u8,
    color: COLOR
}

struct Buffer {
    data: [[Character; BUFFER_WIDTH]; BUFFER_HEIGHT]
}

pub struct Writer {
    column_position: usize,
    color: COLOR,
    buffer: &'static mut Buffer
}

impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b"/n" => self.new_line(),
            _ => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;
                let column = &self.column_position;
                let color = self.color;

                self.buffer.data[row][column] = Character {
                    ascii_character: byte,
                    color
                };

                self.column_position += 1;
            }
        }
    }

    pub fn new_line(&mut self) { /* TODO */ }
}