use core::ptr::{write_volatile, read_volatile};
use super::color::{Color, ColorCode};

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct ScreenChar {
    pub ascii_character: u8,
    pub color_code: ColorCode,
}

impl Default for ScreenChar {
    fn default() -> Self {
        Self {
            ascii_character: b' ',
            color_code: ColorCode::new(Color::White, Color::Black),
        }
    }
}

#[repr(transparent)]
pub struct Buffer {
    pub chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

impl Default for Buffer {
    fn default() -> Self {
        Buffer {
            chars: [[ScreenChar::default(); BUFFER_WIDTH]; BUFFER_HEIGHT],
        }
    }
}

impl Buffer {
    #[inline] pub fn from_raw_pointer(pointer: usize) -> &'static mut Buffer {
        unsafe { &mut *(pointer as *mut Buffer) }
    }

    #[inline] pub fn width(&self) -> usize {
        self.chars[0].len()
    }

    #[inline] pub fn height(&self) -> usize {
        self.chars.len()
    }

    #[inline] pub fn last_line_idx(&self) -> usize {
        self.height() - 1
    }

    #[inline] pub fn write_at(&mut self, row: usize, col: usize, sc: ScreenChar) {
        unsafe {
            write_volatile((&mut self.chars[row][col]) as *mut ScreenChar, sc);
        }
    }

    #[inline] pub fn read_at(&self, row: usize, col: usize) -> ScreenChar {
        unsafe {
            read_volatile((&self.chars[row][col]) as *const ScreenChar)
        }
    }

    pub fn scroll_up(&mut self) {
        for row in 1..self.height() {
            for col in 0..self.width() {
                let character = self.read_at(row, col);
                self.write_at(row - 1, col, character);
            }
        }
    }

    pub fn clear_row(&mut self, row: usize) {
        for col in 0..self.width() {
            self.write_at(row, col, ScreenChar::default());
        }
    }
}
