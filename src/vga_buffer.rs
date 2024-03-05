use volatile::Volatile;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);
impl ColorCode {
    fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

#[repr(transparent)]
struct Buffer {
    chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
    column_postition: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}
impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            // if the new line character, insert a new line
            b'\n' => self.new_line(),
            // if it's an actual byte that we can print, lets do it
            byte => {
                if self.column_postition >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.column_postition;

                let color_code = self.color_code;
                self.buffer.chars[row][col] = ScreenChar {
                    ascii_character: byte,
                    color_code,
                };
                self.column_postition += 1;
            }
        }
    }
    fn new_line(&mut self) {
        todo!()
    }
    // now that we can print one byte, we can also print strings!
    // strings, after all, are just series of bytes
    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                // if new line, or within the printable ascii range,
                // we will call the write_byte function we implemented earlier
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                _ => self.write_byte(0xfe), // not within the range of ASCII characters
            }
        }
    }
}

pub fn print_something() {
    let mut writer = Writer {
        column_postition: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    };
    writer.write_string("Hello world!");
}
