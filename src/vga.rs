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