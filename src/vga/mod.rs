mod color;
mod buffer;
mod writer;

pub use color::{Color, ColorCode};
pub use writer::Writer;

use spin::Mutex;
use lazy_static::lazy_static;
use core::fmt;

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        col_position: 0,
        row_position: 0,
        color_code: ColorCode::new(Color::White, Color::Black),
        buffer: buffer::Buffer::from_raw_pointer(0xb8000) ,
    });
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    use x86_64::instructions::interrupts; 

    interrupts::without_interrupts(|| {  // Disable interruptions to avoid Deadlocks at WRITER
        WRITER.lock().write_fmt(args).expect("Printing to serial failed");
    });
}

#[macro_export]
macro_rules! eprintln {
    () => ({
        $crate::eprint!("\n")
    });
    ($fmt:expr) => ({
        $crate::eprint!(concat!($fmt, "\n"))
    });
    ($fmt:expr, $($arg:tt)*) => ({
        $crate::eprint!(concat!($fmt, "\n"), $($arg)*)
    });
}

#[macro_export]
macro_rules! eprint {
    ($($arg:tt)*) => ({
        $crate::vga::_print_with_color(
            format_args!($($arg)*),
            $crate::vga::ColorCode::new(
                $crate::vga::Color::Red,
                $crate::vga::Color::Yellow,
            ),
        )
    });
}

#[doc(hidden)]
pub fn _print_with_color(args: fmt::Arguments, color: ColorCode) {
    use core::fmt::Write;
    let mut writer = WRITER.lock();
    let old_color = writer.color_code;
    writer.set_color(color);
    writer.write_fmt(args).unwrap();
    writer.set_color(old_color);
}
