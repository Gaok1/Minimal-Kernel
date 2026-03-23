use core::fmt;
use super::color::{Color, ColorCode};
use super::buffer::{Buffer, ScreenChar};

const BUFFER_WIDTH: usize = 80;

pub struct Writer {
    pub col_position: usize,
    pub row_position: usize,
    pub color_code: ColorCode,
    pub buffer: &'static mut Buffer,
}

impl Writer {
    pub fn new(buffer: &'static mut Buffer) -> Self {
        Writer {
            col_position: 0,
            row_position: 0,
            color_code: ColorCode::new(Color::White, Color::Black),
            buffer,
        }
    }

    fn new_line(&mut self) {
        self.row_position += 1;
        self.col_position = 0;
        if self.row_position >= self.buffer.height() {
            self.buffer.scroll_up();
            self.row_position = self.buffer.height() - 1;
            self.buffer.clear_row(self.row_position);
        }
    }

    pub fn write_byte(&mut self, byte: u8, color_opt: Option<ColorCode>) {
        let color_code = color_opt.unwrap_or(self.color_code);
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.col_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = self.row_position;
                let col = self.col_position;
                let new_char = ScreenChar {
                    ascii_character: byte,
                    color_code,
                };

                self.buffer.write_at(row, col, new_char);
                self.col_position += 1;
            }
        }
    }

    pub fn set_color(&mut self, color: ColorCode) {
        self.color_code = color;
    }

    pub fn write_string(&mut self, s: &str, color_opt: Option<ColorCode>) {
        for byte in s.bytes() {
            match byte {
                0x20..=0x7e | b'\n' => self.write_byte(byte, color_opt),
                _ => self.write_byte(0xfe, color_opt),
            }
        }
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s, None);
        Ok(())
    }
}
